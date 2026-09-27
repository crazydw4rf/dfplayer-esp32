use derive_more::{TryFrom, TryFromReprError};
use esp_idf_hal::sys::EspError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    /// A command argument or parameter was outside its valid range.
    #[error("invalid command parameter: {0}")]
    InvalidParameter(&'static str),

    /// A low-level UART communication error from the ESP-IDF driver.
    #[error("failed to send uart command: {0}")]
    UartSendError(#[from] EspError),

    /// Failed to convert a raw byte or number into a recognized enum variant.
    #[error("failed to convert enum from number: {0}")]
    EnumConvertError(#[from] TryFromReprError<u8>),

    /// An error reported by the DFPlayer Mini hardware module in response to a command.
    #[error("device error: {0}")]
    DeviceError(#[from] DeviceError),
}

#[derive(Debug, Default, Clone, Copy, Error, TryFrom, PartialEq, Eq)]
#[try_from(repr)]
#[repr(u8)]
pub enum DeviceError {
    /// Module is busy because initialization has not completed.
    #[error("module is busy, initialization not complete")]
    ModuleBusy = 0x01,

    /// Module is currently in sleep mode.
    ///
    /// Only supported devices can be operated in this mode.
    #[error("module is in sleep mode")]
    SleepMode = 0x02,

    /// A serial frame has not been received completely yet.
    #[error("serial frame received incomplete")]
    IncompleteFrame = 0x03,

    /// The received frame has an invalid checksum.
    #[error("invalid checksum")]
    ChecksumIncorrect = 0x04,

    /// The specified track is outside the current track range.
    #[error("track out of scope")]
    TrackOutOfScope = 0x05,

    /// The specified track could not be found.
    #[error("track not found")]
    TrackNotFound = 0x06,

    /// An insertion operation was requested when no track is being played.
    #[error("advertisement insertion error")]
    InsertionError = 0x07,

    /// Reading from the SD card failed.
    ///
    /// This may occur when the SD card has been removed or is damaged.
    #[error("SD card read failed")]
    SdCardReadFailed = 0x08,

    /// The module has entered sleep mode.
    #[error("entered sleep mode")]
    EnteredSleepMode = 0x0A,

    /// Unknown error.
    #[default]
    #[error("unknown error")]
    Unknown = 0xFF,
}
