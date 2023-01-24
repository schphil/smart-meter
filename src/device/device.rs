use serde::{Deserialize, Serialize};

use super::landis_gyr_t550::LandGyrT550Data;

pub enum Device {
    LandGyrT550,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub enum DeviceData {
    LandGyrT550(LandGyrT550Data),
}
