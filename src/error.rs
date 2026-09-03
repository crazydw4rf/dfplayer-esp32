use esp_idf_hal::sys::EspError;
use thiserror::Error;

#[derive(Error, Debug, Clone, Copy)]
pub enum Error {
    #[error("invalid parameter")]
    InvalidParameter,
    #[error("failed to send uart command: {0}")]
    UartSendError(#[from] EspError),
}
