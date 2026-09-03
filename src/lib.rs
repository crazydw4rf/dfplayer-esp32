use esp_idf_hal::uart::UartDriver;

pub mod command;
pub mod error;
pub mod parameter;

pub use command::*;
pub use parameter::*;

const START_BYTE: u8 = 0x7E;
const END_BYTE: u8 = 0xEF;
const VERSION_BYTE: u8 = 0xFF;
const COMMAND_LENGTH: u8 = 0x06;
const COMMAND_FEEDBACK: u8 = 0x00;

type Result<T, E = error::Error> = std::result::Result<T, E>;

pub struct DfPlayer<'a> {
    uart: UartDriver<'a>,
}

impl<'d> DfPlayer<'d> {
    pub fn new(uart: UartDriver<'d>) -> Self {
        Self { uart }
    }

    pub fn send_command<T>(&self, command: Command, parameter: T) -> Result<()>
    where
        T: TryInto<Parameter>,
        error::Error: From<T::Error>,
    {
        let (param1, param2) = parameter.try_into()?.into();

        let checksum = (VERSION_BYTE as u16
            + COMMAND_LENGTH as u16
            + (command as u8) as u16
            + COMMAND_FEEDBACK as u16
            + param1 as u16
            + param2 as u16)
            .wrapping_neg();

        let command_line: [u8; 10] = [
            START_BYTE,
            VERSION_BYTE,
            COMMAND_LENGTH,
            command as u8,
            COMMAND_FEEDBACK,
            param1,
            param2,
            (checksum >> 8) as u8,   // high byte
            (checksum & 0xFF) as u8, // low byte
            END_BYTE,
        ];

        log::debug!("UART command sent: {:02x?}", command_line);

        self.uart.write(&command_line)?;

        Ok(())
    }
}
