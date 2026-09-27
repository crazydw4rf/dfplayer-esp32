use std::time::Duration;
use crate::error::DeviceError;

use esp_idf_hal::{
    delay::{FreeRtos, TickType},
    uart::UartDriver,
};


pub mod command;
pub mod error;
pub mod payload;
pub mod query;

pub use command::*;
pub use payload::*;
pub use query::*;

const START_BYTE: u8 = 0x7E;
const END_BYTE: u8 = 0xEF;
const VERSION_BYTE: u8 = 0xFF;
const COMMAND_LENGTH: u8 = 0x06;
const RESPONSE_ERROR: u8 = 0x40;

type Result<T, E = error::Error> = std::result::Result<T, E>;

#[derive(PartialEq, Debug)]
pub(crate) enum CommandType {
    Command,
    Query,
}

pub(crate) trait CommandPayload {
    const TYPE: CommandType;

    fn command_byte(self) -> u8;
    fn parameter(self) -> Result<FramePayload, error::Error>;
}

/// Driver for the DFPlayer Mini MP3 module.
///
/// ## Example
///
/// ```rust,no_run
/// use dfplayer_esp32::DfPlayer;
/// use esp_idf_hal::peripherals::Peripherals;
/// use esp_idf_hal::uart::{config::Config as UartConfig, UartDriver};
/// use esp_idf_hal::gpio::AnyIOPin;
///
/// let peripherals = Peripherals::take().unwrap();
/// let config = UartConfig::new().baudrate(9600.into());
///
/// let uart = UartDriver::new(
///     peripherals.uart1,
///     peripherals.pins.gpio17, // TX pin connected to DFPlayer RX
///     peripherals.pins.gpio16, // RX pin connected to DFPlayer TX
///     Option::<AnyIOPin>::None,
///     Option::<AnyIOPin>::None,
///     &config,
/// ).unwrap();
///
/// let player = DfPlayer::new(&uart);
/// player.set_volume(15).unwrap();
/// player.play_track(1).unwrap();
/// ```
pub struct DfPlayer<'d> {
    uart: &'d UartDriver<'d>,
    timeout: TickType,
}

impl<'d> DfPlayer<'d> {
    /// Creates a new DFPlayer Mini driver instance using the provided UART driver.
    ///
    /// The default response timeout is set to 1000 milliseconds.
    pub fn new(uart: &'d UartDriver<'d>) -> Self {
        Self {
            uart,
            timeout: TickType::new_millis(1000), // default 1000 milliseconds
        }
    }

    /// Sets the response timeout duration for receiving response and query frames.
    pub fn set_timeout(&mut self, timeout: Duration) {
        self.timeout = TickType::from(timeout);
    }

    pub(crate) fn send_command<C, T>(&self, command: C) -> Result<T>
    where
        C: CommandPayload + Clone + Copy,
        T: TryFrom<FramePayload, Error = error::Error>,
    {
        let (param1, param2) = command.parameter()?.into();
        let command_byte = command.command_byte();

        let ack = (C::TYPE == CommandType::Command) as u8;

        let [checksum_high, checksum_low] = (VERSION_BYTE as u16
            + COMMAND_LENGTH as u16
            + command_byte as u16
            + ack as u16
            + param1 as u16
            + param2 as u16)
            .wrapping_neg()
            .to_be_bytes();

        let frame: [u8; 10] = [
            START_BYTE,
            VERSION_BYTE,
            COMMAND_LENGTH,
            command_byte,
            ack,
            param1,
            param2,
            checksum_high,
            checksum_low,
            END_BYTE,
        ];

        self.uart.clear_rx()?;

        log::debug!("Sending frame: {frame:02X?}");
        self.uart.write(&frame)?;

        let mut frame = [0u8; 10];
        self.uart.read(&mut frame, self.timeout.ticks())?;
        log::debug!("Received frame: {frame:02X?}");

        let (command, lsb) = (frame[3], frame[6]);

        if command == RESPONSE_ERROR {
            let error = DeviceError::try_from(lsb).unwrap_or_default();
            log::error!("DFPlayer returned error: {error}");
            return Err(error::Error::DeviceError(error));
        }

        let data = FramePayload::from(frame).try_into().map_err(|e| {
            log::error!("Failed to parse response payload: {e}");
            e
        })?;

        Ok(data)
    }

    /// Plays the next track.
    pub fn play_next(&self) -> Result<()> {
        self.send_command(Command::Next)
    }

    /// Plays the previous track.
    pub fn play_previous(&self) -> Result<()> {
        self.send_command(Command::Previous)
    }

    /// Plays a specific track in the root folder (1–3000).
    pub fn play_track(&self, track_num: u16) -> Result<()> {
        self.send_command(Command::PlayTrack(track_num))
    }

    /// Sets the equalizer preset.
    pub fn set_equalizer_preset(&self, preset: EqualizerPreset) -> Result<()> {
        self.send_command(Command::SetEqualizerPreset(preset))
    }

    /// Increases the volume by one step.
    pub fn increase_volume(&self) -> Result<()> {
        self.send_command(Command::VolumeUp)
    }

    /// Decreases the volume by one step.
    pub fn decrease_volume(&self) -> Result<()> {
        self.send_command(Command::VolumeDown)
    }

    /// Sets the volume level (0–30).
    pub fn set_volume(&self, volume: u8) -> Result<()> {
        self.send_command(Command::SetVolume(volume))
    }

    /// Repeats a single track (1–3000).
    pub fn repeat_track(&self, track_num: u16) -> Result<()> {
        self.send_command(Command::RepeatTrack(track_num))
    }

    /// Enables or disables repeat playback for all tracks.
    pub fn repeat_all(&self, state: SwitchState) -> Result<()> {
        self.send_command(Command::RepeatAll(state))
    }

    /// Sets the playback source (USB, TF Card, AUX, Sleep, Flash).
    pub fn set_playback_source(&self, source: PlaybackSource) -> Result<()> {
        self.send_command(Command::SetPlaybackSource(source))
    }

    /// Enters low-power standby mode.
    pub fn enter_standby(&self) -> Result<()> {
        self.send_command(Command::EnterStandby)
    }

    /// Exits standby mode and resumes normal operation.
    pub fn exit_standby(&self) -> Result<()> {
        self.send_command(Command::ExitStandby)
    }

    /// Resets the DFPlayer Mini module.
    ///
    /// Note that this function takes approximately 1.5 seconds to allow the module
    /// to complete its hardware reboot and mount storage devices.
    pub fn reset_module(&self) -> Result<()> {
        log::debug!("Resetting the DFPlayer Mini module...");
        self.send_command::<_, ()>(Command::ResetModule)?;

        FreeRtos::delay_ms(1500);
        self.uart.clear_rx()?;

        Ok(())
    }

    /// Resumes playback.
    pub fn resume(&self) -> Result<()> {
        self.send_command(Command::Resume)
    }

    /// Pauses playback.
    pub fn pause(&self) -> Result<()> {
        self.send_command(Command::Pause)
    }

    /// Plays a track from a specific folder (Folder 1–99, File 1–255).
    pub fn play_from_folder(&self, folder: u8, track: u8) -> Result<()> {
        self.send_command(Command::PlayFromFolder(folder, track))
    }

    /// Plays a track from a specific folder (Folder 1–15, File 1–3000).
    pub fn play_from_folder_large(&self, folder: u8, track: u16) -> Result<()> {
        self.send_command(Command::PlayFromFolderLarge(folder, track))
    }

    /// Sets the volume gain (0–31).
    pub fn set_volume_gain(&self, state: SwitchState, gain: u8) -> Result<()> {
        self.send_command(Command::SetVolumeGain(state, gain))
    }

    /// Plays a specific track in the MP3 folder (1–3000).
    pub fn play_from_mp3_folder(&self, track_num: u16) -> Result<()> {
        self.send_command(Command::PlayFromMp3Folder(track_num))
    }

    /// Plays a specific track in the ADVERT folder (1–3000).
    pub fn play_from_advert_folder(&self, track_num: u16) -> Result<()> {
        self.send_command(Command::PlayFromAdvertFolder(track_num))
    }

    /// Stops the currently playing advertisement and resumes previous playback.
    pub fn skip_advertisement(&self) -> Result<()> {
        self.send_command(Command::SkipAdvertisement)
    }

    /// Stops all playback.
    pub fn stop_all_playback(&self) -> Result<()> {
        self.send_command(Command::StopAllPlayback)
    }

    /// Plays and repeats all tracks in a specific folder (Folder 1–99).
    pub fn repeat_playback_folder(&self, folder: u8) -> Result<()> {
        self.send_command(Command::RepeatPlaybackFolder(folder))
    }

    /// Starts random playback for all tracks.
    pub fn start_random_playback(&self) -> Result<()> {
        self.send_command(Command::StartRandomPlayback)
    }

    /// Enables or disables repeating the currently playing track.
    pub fn repeat_current_playback(&self, state: SwitchState) -> Result<()> {
        self.send_command(Command::RepeatCurrentPlayback(state))
    }

    /// Sets the DAC state (on or off).
    pub fn set_dac_state(&self, state: SwitchState) -> Result<()> {
        self.send_command(Command::SetDacState(state))
    }

    /// Sends a raw command byte with two parameter bytes.
    pub fn send_raw(&self, command: u8, param1: u8, param2: u8) -> Result<()> {
        self.send_command(Command::Raw(command, param1, param2))
    }

    /// Queries the current active playback storage source (USB, TF Card, AUX, Sleep, Flash).
    pub fn current_playback_source(&self) -> Result<PlaybackSource> {
        self.send_command(Query::PlaybackSource)
    }

    /// Queries the current playback status (Playing, Paused, Stopped, or Sleep).
    pub fn current_playback_status(&self) -> Result<PlaybackStatus> {
        self.send_command(Query::PlaybackStatus)
    }

    /// Queries the current output volume level (0–30).
    pub fn volume_level(&self) -> Result<u8> {
        self.send_command(Query::VolumeLevel)
    }

    /// Queries the current equalizer preset (Normal, Pop, Rock, Jazz, Classic, Bass).
    pub fn current_equalizer_preset(&self) -> Result<EqualizerPreset> {
        self.send_command(Query::EqualizerPreset)
    }

    /// Queries the total number of audio tracks on the USB flash drive.
    pub fn tracks_count_usb(&self) -> Result<u16> {
        self.send_command(Query::TracksCountUsb)
    }

    /// Queries the total number of audio tracks on the TF (MicroSD) card.
    pub fn tracks_count_tf(&self) -> Result<u16> {
        self.send_command(Query::TracksCountTf)
    }

    /// Queries the currently playing track number on the USB flash drive.
    pub fn current_track_usb(&self) -> Result<u16> {
        self.send_command(Query::CurrentTrackUsb)
    }

    /// Queries the currently playing track number on the TF (MicroSD) card.
    pub fn current_track_tf(&self) -> Result<u16> {
        self.send_command(Query::CurrentTrackTf)
    }

    /// Queries the total number of audio tracks in a specific folder (Folder 1–99).
    pub fn tracks_count_in_folder(&self, folder: u8) -> Result<u16> {
        self.send_command(Query::TrackCountsFolder(folder))
    }
}
