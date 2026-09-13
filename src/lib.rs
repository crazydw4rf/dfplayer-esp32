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

pub struct DfPlayer<'d> {
    uart: UartDriver<'d>,
}

impl<'d> DfPlayer<'d> {
    pub fn new(uart: UartDriver<'d>) -> Self {
        Self { uart }
    }

    pub fn send_command(&self, command: Command) -> Result<()> {
        let (param1, param2) = command.parameter()?.into();
        let command_byte = command.command_byte();

        let [checksum_high, checksum_low] = (VERSION_BYTE as u16
            + COMMAND_LENGTH as u16
            + command_byte as u16
            + COMMAND_FEEDBACK as u16
            + param1 as u16
            + param2 as u16)
            .wrapping_neg()
            .to_be_bytes();

        let command_line: [u8; 10] = [
            START_BYTE,
            VERSION_BYTE,
            COMMAND_LENGTH,
            command_byte,
            COMMAND_FEEDBACK,
            param1,
            param2,
            checksum_high,
            checksum_low,
            END_BYTE,
        ];

        log::debug!("UART command sent: {:02X?}", command_line);

        self.uart.write(&command_line)?;

        Ok(())
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
    pub fn reset_module(&self) -> Result<()> {
        self.send_command(Command::ResetModule)
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
}
