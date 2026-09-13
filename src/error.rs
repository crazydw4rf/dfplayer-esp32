use esp_idf_hal::sys::EspError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("invalid command parameter: {0}")]
    InvalidParameter(&'static str),
    #[error("failed to send uart command: {0}")]
    UartSendError(#[from] EspError),
}
