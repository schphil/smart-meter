#[cfg(esp_std)]
use std::sync::{Arc, Mutex};

use super::{
    device::{device::*, landis_gyr_t550::*},
    protocol::en6205621,
};

use super::error::Error;

#[cfg(esp)]
use core::cell::RefCell;
#[cfg(esp)]
use critical_section::Mutex;
#[cfg(esp)]
use esp32c3_hal::{
    clock::Clocks,
    gpio::{Bank0GpioRegisterAccess, Floating, GpioPin, Input, InputOutputAnalogPinType, Output, PushPull},
    pac::UART1,
    serial::TxRxPins,
};
#[cfg(esp_std)]
use esp_idf_hal::{gpio, uart};

pub enum Protocol {
    EN6205621,
}

#[cfg(default)]
pub enum Interface {
    Serial(String),
}

#[cfg(esp_std)]
pub enum Interface {
    Serial(
        Arc<Mutex<gpio::Gpio4>>,
        Arc<Mutex<gpio::Gpio5>>,
        Arc<Mutex<uart::UART1>>,
    ),
}

#[cfg(esp)]
pub enum Interface {
    Serial(
        Mutex<
            RefCell<
                Option<
                    TxRxPins<
                        GpioPin<Output<PushPull>, Bank0GpioRegisterAccess, InputOutputAnalogPinType, 4>,
                        GpioPin<Input<Floating>, Bank0GpioRegisterAccess, InputOutputAnalogPinType, 5>,
                    >,
                >,
            >,
        >,
        Mutex<RefCell<Option<UART1>>>,
        Mutex<RefCell<Clocks>>,
    ),
}

pub struct SmartMeter {
    pub device: Device,
    pub protocol: Protocol,
    pub interface: Interface,
}

impl SmartMeter {
    // async fn init() {}
    #[cfg(default)]
    pub async fn request_data(&self) -> Result<DeviceData, Error> {
        match self.protocol {
            Protocol::EN6205621 => match &self.interface {
                Interface::Serial(_) => {
                    let device_information = en6205621::request(&self).await?;
                    let data: LandGyrT550Data = en6205621::read_data(&self, device_information.protocol_mode)
                        .await
                        .unwrap()
                        .try_into()?;
                    Ok(DeviceData::LandGyrT550(data))
                },
            },
        }
    }

    #[cfg(esp_std)]
    pub fn request_data(&self) -> Result<DeviceData, Error> {
        match self.protocol {
            Protocol::EN6205621 => match &self.interface {
                Interface::Serial(_, _, _) => {
                    let device_information = en6205621::request(&self).unwrap();
                    let data: LandGyrT550Data =
                        en6205621::read_data(&self, device_information.protocol_mode)?.try_into()?;
                    Ok(DeviceData::LandGyrT550(data))
                },
            },
        }
    }

    #[cfg(esp)]
    pub fn request_data(&self) -> Result<DeviceData, Error> {
        match self.protocol {
            Protocol::EN6205621 => match &self.interface {
                Interface::Serial(_, _, _) => {
                    let device_information = en6205621::request(&self)?;
                    let data: LandGyrT550Data =
                        en6205621::read_data(&self, device_information.protocol_mode)?.try_into()?;
                    Ok(DeviceData::LandGyrT550(data))
                },
            },
        }
    }
}
