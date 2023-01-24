#[cfg(esp)]
use heapless::String;

#[cfg(any(default, esp_std))]
use serde::{Deserialize, Serialize};

#[cfg(esp)]
use serde::{Deserialize, Serialize};

use super::super::error::Error;

#[cfg(any(default, esp_std))]
#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct LandGyrT550Data {
    pub accumulated_energy: f64,
    pub throughput: f64,
    pub operating_time: u16,
    pub failure_time: u16,
    pub operating_time_with_throughput: u16,
    pub property_number: u32,
}

#[cfg(esp)]
#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct LandGyrT550Data {
    pub accumulated_energy: f64,
    pub throughput: f64,
    pub operating_time: u16,
    pub failure_time: u16,
    pub operating_time_with_throughput: u16,
    pub property_number: u32,
}

#[cfg(any(default, esp_std))]
impl TryFrom<String> for LandGyrT550Data {
    type Error = Error;

    fn try_from(s: String) -> Result<LandGyrT550Data, Error> {
        let accumulated_energy = s.find("6.8(").ok_or(Error::ParseLandisGyrT550DataError)?;
        let accumulated_energy = s[accumulated_energy + 4..accumulated_energy + 4 + 8].parse::<f64>()?;
        let throughput = s.find("6.26(").ok_or(Error::ParseLandisGyrT550DataError)?;
        let throughput = s[throughput + 5..throughput + 5 + 8].parse::<f64>()?;
        let property_number = s.find("9.21(").ok_or(Error::ParseLandisGyrT550DataError)?;
        let property_number = s[property_number + 5..property_number + 5 + 8].parse::<u32>()?;
        let operating_time = s.find("6.31(").ok_or(Error::ParseLandisGyrT550DataError)?;
        let operating_time = s[operating_time + 5..operating_time + 5 + 7].parse::<u16>()?;
        let failure_time = s.find("6.32(").ok_or(Error::ParseLandisGyrT550DataError)?;
        let failure_time = s[failure_time + 5..failure_time + 5 + 7].parse::<u16>()?;
        let operating_time_with_throughput = s.find("9.31(").ok_or(Error::ParseLandisGyrT550DataError)?;
        let operating_time_with_throughput =
            s[operating_time_with_throughput + 5..operating_time_with_throughput + 5 + 7].parse::<u16>()?;

        Ok(LandGyrT550Data {
            accumulated_energy,
            throughput,
            operating_time,
            failure_time,
            operating_time_with_throughput,
            property_number,
        })
    }
}

#[cfg(esp)]
impl TryFrom<String<1042>> for LandGyrT550Data {
    type Error = Error;

    fn try_from(s: String<1042>) -> Result<LandGyrT550Data, Error> {
        let accumulated_energy = s.find("6.8(").ok_or(Error::ParseLandisGyrT550DataError)?;
        let accumulated_energy = s[accumulated_energy + 4..accumulated_energy + 4 + 8].parse::<f64>()?;
        let throughput = s.find("6.26(").ok_or(Error::ParseLandisGyrT550DataError)?;
        let throughput = s[throughput + 5..throughput + 5 + 8].parse::<f64>()?;
        let property_number = s.find("9.21(").ok_or(Error::ParseLandisGyrT550DataError)?;
        let property_number = s[property_number + 5..property_number + 5 + 8].parse::<u32>()?;
        let operating_time = s.find("6.31(").ok_or(Error::ParseLandisGyrT550DataError)?;
        let operating_time = s[operating_time + 5..operating_time + 5 + 7].parse::<u16>()?;
        let failure_time = s.find("6.32(").ok_or(Error::ParseLandisGyrT550DataError)?;
        let failure_time = s[failure_time + 5..failure_time + 5 + 7].parse::<u16>()?;
        let operating_time_with_throughput = s.find("9.31(").ok_or(Error::ParseLandisGyrT550DataError)?;
        let operating_time_with_throughput =
            s[operating_time_with_throughput + 5..operating_time_with_throughput + 5 + 7].parse::<u16>()?;

        Ok(LandGyrT550Data {
            accumulated_energy,
            throughput,
            operating_time,
            failure_time,
            operating_time_with_throughput,
            property_number,
        })
    }
}

#[test]
fn string_to_t550_data() {
    let data = "\u{2}6.8(0001.015*MWh)6.26(00036.23*m3)9.21(71708961)\r\n6.26*01(00000.00*m3)6.8*01(0000.000*MWh)\r\nF(0)9.20(71708961)6.35(60*m)\r\n6.6(0008.9*kW)6.6*01(0000.0*kW)6.33(000.312*m3ph)9.4(077.6*C&069.0*C)\r\n6.31(0001545*h)6.32(0000000*h)9.22(R)9.6(000&71708961&0&000&71708961&0)\r\n9.7(20000)6.32*01(0000000*h)6.36(01-01&00:00)6.33*01(000.000*m3ph)\r\n6.8.1()6.8.2()6.8.3()6.8.4()6.8.5()\r\n6.8.1*01()6.8.2*01()6.8.3*01()\r\n6.8.4*01()6.8.5*01()\r\n9.4*01(000.0*C&000.0*C)\r\n6.36.1(2022-12-18)6.36.1*01(2000-00-00)\r\n6.36.2(2022-12-18)6.36.2*01(2000-00-00)\r\n6.36.3(2022-12-14)6.36.3*01(2000-00-00)\r\n6.36.4(2022-12-14)6.36.4*01(2000-00-00)\r\n6.36.5()6.36*02(01&00:00)9.36(2022-12-25&01:23:39)9.24(0.6*m3ph)\r\n9.17(0)9.18()9.19()9.25()\r\n9.1(0&1&0&0017&CECV&CECV&1&5.24&5.24&F&101008&1>1>04&08&0&00&:5&00&20)\r\n9.2(&&)9.29()9.31(0000275*h)\r\n9.0.1(00000000)9.0.2(00000000)9.34.1(000.00000*m3)9.34.2(000.00000*m3)\r\n8.26.1(00000000*m3)8.26.2(00000000*m3)\r\n8.26.1*01(00000000*m3)8.26.2*01(00000000*m3)\r\n6.26.1()6.26.4()6.26.5()\r\n6.26.1*01()6.26.4*01()6.26.5*01()0.0(71708961)\r\n".to_string();

    let t550_data = LandGyrT550Data {
        accumulated_energy: 1.015,
        throughput: 36.23,
        operating_time: 1545,
        failure_time: 0,
        operating_time_with_throughput: 275,
        property_number: 71708961,
    };

    assert_eq!(t550_data, data.try_into().unwrap());
}

//     let data =
// "\u{2}6.8(0001.823*MWh)6.26(00066.00*m3)9.21(71708961)\r\n6.26*01(00052.96*m3)6.8*01(0001.467*MWh)\r\nF(0)9.
// 20(71708961)6.35(60*m)\r\n6.6(0009.4*kW)6.6*01(0009.4*kW)6.33(000.312*m3ph)9.4(077.6*C&069.0*C)\r\n6.31(0001847*h)6.
// 32(0000000*h)9.22(R)9.6(000&71708961&0&000&71708961&0)\r\n9.7(20000)6.32*01(0000000*h)6.36(01-01&00:00)6.33*01(000.
// 312*m3ph)\r\n6.8.1()6.8.2()6.8.3()6.8.4()6.8.5()\r\n6.8.1*01()6.8.2*01()6.8.3*01()\r\n6.8.4*01()6.8.5*01()\r\n9.4*
// 01(077.6*C&069.0*C)\r\n6.36.1(2022-12-25)6.36.1*01(2022-12-25)\r\n6.36.2(2022-12-18)6.36.2*01(2022-12-18)\r\n6.36.
// 3(2022-12-14)6.36.3*01(2022-12-14)\r\n6.36.4(2022-12-14)6.36.4*01(2022-12-14)\r\n6.36.5()6.36*02(01&00:00)9.
// 36(2023-01-06&15:12:41)9.24(0.6*m3ph)\r\n9.17(0)9.18()9.19()9.25()\r\n9.1(0&1&0&0017&CECV&CECV&1&5.24&5.24&F&101008&
// 1>1>04&08&0&00&:5&00&20)\r\n9.2(&&)9.29()9.31(0000577*h)\r\n9.0.1(00000000)9.0.2(00000000)9.34.1(000.00000*m3)9.34.
// 2(000.00000*m3)\r\n8.26.1(00000000*m3)8.26.2(00000000*m3)\r\n8.26.1*01(00000000*m3)8.26.2*01(00000000*m3)\r\n6.26.
// 1()6.26.4()6.26.5()\r\n6.26.1*01()6.26.4*01()6.26.5*01()0.0(71708961)\r\n".to_string();
