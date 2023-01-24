use std::sync::{Arc, Mutex};

use esp_idf_sys as _; // If using the `binstart` feature of `esp-idf-sys`, always keep this module imported

use esp_idf_hal::{gpio, peripherals::Peripherals, uart};

use smart_meter::{
    device::device::Device,
    error::Error,
    smart_meter::{Interface, Protocol, SmartMeter},
};

fn main() -> Result<(), Error> {
    esp_idf_sys::link_patches();

    let peripherals = Peripherals::take().unwrap();
    let uart1_mutex: Arc<Mutex<uart::UART1>> = Arc::new(Mutex::new(peripherals.uart1));
    let tx_mutex: Arc<Mutex<gpio::Gpio4>> = Arc::new(Mutex::new(peripherals.pins.gpio4));
    let rx_mutex: Arc<Mutex<gpio::Gpio5>> = Arc::new(Mutex::new(peripherals.pins.gpio5));

    let smart_meter = SmartMeter {
        device: Device::LandGyrT550,
        protocol: Protocol::EN6205621,
        interface: Interface::Serial(tx_mutex, rx_mutex, uart1_mutex),
    };

    let data = smart_meter.request_data().unwrap();
    println!("Device data: {:?}", data);

    Ok(())
}
