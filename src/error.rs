#[cfg(any(default, esp_std))]
use thiserror::Error;

#[cfg(esp)]
use thiserror_no_std::Error;

#[cfg(any(default, esp_std))]
#[derive(Debug, Error)]
pub enum Error {
    #[error("baudrate convert Error")]
    BaudrateConvertError,
    #[cfg(target_arch = "riscv32")]
    #[error("esp Error")]
    EspError(#[from] esp_idf_sys::EspError),
    #[error("parse landis gyr data Error")]
    ParseLandisGyrT550DataError,
    #[error("parse float Error")]
    ParseFloatError(#[from] std::num::ParseFloatError),
    #[error("parse int Error")]
    ParseIntError(#[from] std::num::ParseIntError),
    #[cfg(esp_std)]
    #[error("Utf Error")]
    UtfError(#[from] std::str::Utf8Error),
    #[error("Std Error")]
    StdError(#[from] std::io::Error),
    #[cfg(not(target_arch = "riscv32"))]
    #[error("tokio serial Error")]
    TokioSerialError(#[from] tokio_serial::Error),
}

#[cfg(esp)]
#[derive(Debug, Error)]
pub enum Error {
    #[error("baudrate convert Error")]
    BaudrateConvertError,
    #[error("parse landis gyr data Error")]
    ParseLandisGyrT550DataError,
    #[error("parse float Error")]
    ParseFloatError(#[from] core::num::ParseFloatError),
    #[error("parse int Error")]
    ParseIntError(#[from] core::num::ParseIntError),
    #[error("esp serial Error")]
    EspSerialError(#[from] esp32c3_hal::serial::Error),
    #[error("Utf Error")]
    UtfError(#[from] core::str::Utf8Error),
}
