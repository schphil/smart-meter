#[cfg(any(default, esp_std))]
use std::str;
#[cfg(default)]
use std::time::Duration;

#[cfg(esp)]
use core::convert::{From, Into};
#[cfg(esp)]
use esp32c3_hal::{
    prelude::*,
    serial::config::{Config, DataBits, Parity, StopBits},
    Serial,
};
#[cfg(esp_std)]
use esp_idf_hal::{
    gpio,
    prelude::*,
    uart::{config, UartDriver},
};
#[cfg(default)]
use futures::StreamExt;
#[cfg(esp)]
use heapless::{String, Vec};
#[cfg(default)]
use tokio::io::AsyncWriteExt;
#[cfg(default)]
use tokio_serial::{Parity, SerialPortBuilderExt, StopBits};
#[cfg(default)]
use tokio_util::codec::Decoder;

use super::super::error::Error;
#[cfg(default)]
use super::super::util::LineCodec;

use super::super::smart_meter::{Interface, SmartMeter};

#[cfg(any(default, esp_std))]
pub enum Message {
    Init,
    Request,
}

#[cfg(any(default, esp_std))]
impl Message {
    #[cfg(any(default, esp_std))]
    pub fn to_bytes(self) -> Vec<u8> {
        match &self {
            Message::Init => vec![
                0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                0x00, 0x00, 0x00,
            ],
            Message::Request => vec![0x2F, 0x3F, 0x21, 0x0D, 0x0A],
        }
    }
}

#[cfg(esp)]
const MESSAGE_INIT: [u8; 20] = [
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00,
];

#[cfg(esp)]
const MESSAGE_REQUEST: [u8; 5] = [0x2F, 0x3F, 0x21, 0x0D, 0x0A];

#[derive(Debug)]
pub enum ProtocolMode {
    A,
    B,
    C,
    D,
    E,
    F,
    Unknown,
}

impl From<&str> for ProtocolMode {
    fn from(str: &str) -> ProtocolMode {
        match str {
            "A" => ProtocolMode::A,
            "B" => ProtocolMode::B,
            "C" => ProtocolMode::C,
            "D" => ProtocolMode::D,
            "E" => ProtocolMode::E,
            "F" => ProtocolMode::F,
            _ => ProtocolMode::Unknown,
        }
    }
}
#[derive(Debug)]
pub struct DeviceInformation {
    #[cfg(any(default, esp_std))]
    pub manufacturer_identification: String,
    #[cfg(esp)]
    pub manufacturer_identification: String<3>,
    pub protocol_mode: ProtocolMode,
}

impl Into<DeviceInformation> for &str {
    fn into(self) -> DeviceInformation {
        DeviceInformation {
            manufacturer_identification: self[1..4].into(),
            protocol_mode: self[4..5].into(),
        }
    }
}

#[cfg(default)]
pub async fn request(smart_meter: &SmartMeter) -> Result<DeviceInformation, Error> {
    let Interface::Serial(path) = &smart_meter.interface;
    let serial_config = tokio_serial::new(path, 300)
        .data_bits(tokio_serial::DataBits::Seven)
        .parity(Parity::Even)
        .stop_bits(StopBits::Two)
        .timeout(std::time::Duration::from_millis(1000));

    let mut serial = serial_config.open_native_async()?;

    // init communication
    serial.write(&Message::Init.to_bytes()).await?;

    // send request message
    serial.write(&Message::Request.to_bytes()).await?;

    let mut reader = LineCodec.framed(serial);

    match reader.next().await.unwrap() {
        Ok(line) => {
            let device_information: DeviceInformation = line.as_str().into();
            #[cfg(not(target_arch = "riscv32"))]
            log::info!("Device information: {:?}", device_information);
            Ok(device_information)
        },
        Err(e) => Err(Error::StdError(e)),
    }
}

#[cfg(esp_std)]
pub fn request(smart_meter: &SmartMeter) -> Result<DeviceInformation, Error> {
    let config = config::Config::new()
        .baudrate(Hertz(300))
        .data_bits(config::DataBits::DataBits7)
        .parity_even()
        .stop_bits(config::StopBits::STOP2);

    let Interface::Serial(tx_mutex, rx_mutex, uart1_mutex) = &smart_meter.interface;

    let uart = UartDriver::new(
        uart1_mutex.lock().unwrap(),
        tx_mutex.lock().unwrap(),
        rx_mutex.lock().unwrap(),
        Option::<gpio::Gpio0>::None,
        Option::<gpio::Gpio1>::None,
        &config,
    )
    .unwrap();

    // init communication
    uart.write(&Message::Init.to_bytes())?;

    // send request message
    uart.write(&Message::Request.to_bytes())?;

    let mut buf = [0_u8; 11];

    let data_string = loop {
        uart.read(&mut buf, 100)?;

        if buf[0] != 0 {
            break std::str::from_utf8(&buf)?;
        }
    };

    let device_information: DeviceInformation = data_string.into();
    println!("Device information: {:?}", device_information);

    Ok(device_information)
}

#[cfg(esp)]
pub fn request(smart_meter: &SmartMeter) -> Result<DeviceInformation, Error> {
    let config = Config {
        baudrate: 300,
        data_bits: DataBits::DataBits7,
        parity: Parity::ParityEven,
        stop_bits: StopBits::STOP2,
    };

    let Interface::Serial(pins, uart, clocks) = &smart_meter.interface;

    let mut uart = critical_section::with(|cs| {
        let uart = uart.borrow(cs).take().unwrap();
        let pins = pins.borrow(cs).take().unwrap();
        let clocks = clocks.borrow(cs).borrow();
        Serial::new_with_config(uart, Some(config), Some(pins), &clocks)
    });

    // init communication
    for byte in MESSAGE_INIT {
        uart.write(byte);
    }

    // send request message
    for byte in MESSAGE_REQUEST {
        uart.write(byte);
    }

    let mut buffer: Vec<u8, 11> = Vec::new();

    loop {
        let buf = nb::block!(uart.read()).unwrap();
        buffer.push(buf).unwrap();
        if buf == 0x0A {
            break;
        }
    }

    let data_string = core::str::from_utf8(&buffer)?;

    Ok(data_string.into())
}

#[cfg(default)]
pub async fn read_data(smart_meter: &SmartMeter, protocol_mode: ProtocolMode) -> Result<String, Error> {
    let baudrate = match protocol_mode {
        ProtocolMode::A => Ok(600),
        ProtocolMode::B => Ok(1200),
        ProtocolMode::C => Ok(2400),
        ProtocolMode::D => Ok(4800),
        ProtocolMode::E => Ok(9600),
        ProtocolMode::F => Ok(19200),
        ProtocolMode::Unknown => Err(Error::BaudrateConvertError),
    }?;

    let Interface::Serial(path) = &smart_meter.interface;

    let config = tokio_serial::new(path, baudrate)
        .data_bits(tokio_serial::DataBits::Seven)
        .parity(Parity::Even)
        .stop_bits(StopBits::Two)
        .timeout(Duration::from_millis(1000));

    let serial = config.open_native_async()?;

    let mut reader = LineCodec.framed(serial);

    let mut data: String = "".to_string();

    while let Some(line_result) = reader.next().await {
        let line = line_result.expect("Failed to read line");
        if line.contains("!") {
            break;
        }
        data.push_str(&line)
    }

    Ok(data)
}

#[cfg(esp_std)]
pub fn read_data(smart_meter: &SmartMeter, protocol_mode: ProtocolMode) -> Result<String, Error> {
    let baudrate = match protocol_mode {
        ProtocolMode::A => Ok(600),
        ProtocolMode::B => Ok(1200),
        ProtocolMode::C => Ok(2400),
        ProtocolMode::D => Ok(4800),
        ProtocolMode::E => Ok(9600),
        ProtocolMode::F => Ok(19200),
        ProtocolMode::Unknown => Err(Error::BaudrateConvertError),
    }?;

    let config = config::Config::new()
        .baudrate(Hertz(baudrate))
        .data_bits(config::DataBits::DataBits7)
        .parity_even()
        .stop_bits(config::StopBits::STOP2);

    let Interface::Serial(tx_mutex, rx_mutex, uart1_mutex) = &smart_meter.interface;

    let uart = UartDriver::new(
        uart1_mutex.lock().unwrap(),
        tx_mutex.lock().unwrap(),
        rx_mutex.lock().unwrap(),
        Option::<gpio::Gpio0>::None,
        Option::<gpio::Gpio1>::None,
        &config,
    )?;

    let data_string = loop {
        let mut buf = [0_u8; 1042];
        uart.read(&mut buf, 100)?;

        if buf[0] != 0 {
            break std::str::from_utf8(&buf)?.to_string();
        }
    };

    Ok(data_string)
}

#[cfg(esp)]
pub fn read_data(smart_meter: &SmartMeter, protocol_mode: ProtocolMode) -> Result<String<1042>, Error> {
    let baudrate = match protocol_mode {
        ProtocolMode::A => Ok(600),
        ProtocolMode::B => Ok(1200),
        ProtocolMode::C => Ok(2400),
        ProtocolMode::D => Ok(4800),
        ProtocolMode::E => Ok(9600),
        ProtocolMode::F => Ok(19200),
        ProtocolMode::Unknown => Err(Error::BaudrateConvertError),
    }?;

    let config = Config {
        baudrate,
        data_bits: DataBits::DataBits7,
        parity: Parity::ParityEven,
        stop_bits: StopBits::STOP2,
    };

    let Interface::Serial(pins, uart, clocks) = &smart_meter.interface;

    let mut uart = critical_section::with(|cs| {
        let uart = uart.borrow(cs).take().unwrap();
        let pins = pins.borrow(cs).take().unwrap();
        let clocks = clocks.borrow(cs).borrow();
        Serial::new_with_config(uart, Some(config), Some(pins), &clocks)
    });

    let mut buffer: Vec<u8, 1042> = Vec::new();

    loop {
        let buf = nb::block!(uart.read()).unwrap();
        buffer.push(buf).unwrap();
        if buf == 0x21 {
            break;
        }
    }

    let data_str = core::str::from_utf8(&buffer)?;
    let data_string: String<1042> = String::from(data_str);

    Ok(data_string)
}
