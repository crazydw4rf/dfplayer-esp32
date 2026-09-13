use crate::PlaybackSource;
use crate::parameter::{EqualizerPreset, Parameter};
use crate::{Result, error::Error};

/// A collection of DFPlayer Mini commands.
#[derive(Clone, Copy)]
pub enum Command {
    /// Plays the next track.
    Next,

    /// Plays the previous track.
    Previous,

    /// Plays a specific track number (1–3000) in the root folder.
    PlayTrack(u16),

    /// Increases the volume by one step.
    VolumeUp,

    /// Decreases the volume by one step.
    VolumeDown,

    /// Sets the volume level (0–30).
    SetVolume(u8),

    /// Sets the equalizer preset (Normal, Pop, Rock, Jazz, Classic, Bass).
    SetEqualizerPreset(EqualizerPreset),

    /// Repeats a single track (1–3000).
    RepeatTrack(u16),

    /// Sets the playback source (USB, TF Card, AUX, Sleep, Flash).
    SetPlaybackSource(PlaybackSource),

    /// Enters low-power standby mode.
    EnterStandby,

    /// Exits standby mode and resumes normal operation.
    ExitStandby,

    /// Resets the DFPlayer Mini module.
    ResetModule,

    /// Resumes playback.
    Resume,

    /// Pauses playback.
    Pause,

    /// Plays a track from a specific folder (Folder 1–99, File 1–255).
    PlayFromFolder(u8, u8),

    /// Sets the volume gain (0–31).
    SetVolumeGain(bool, u8),

    /// Enables or disables repeat playback for all tracks.
    RepeatAll(bool),

    /// Plays a specific track (1–3000) in the MP3 folder.
    PlayFromMp3Folder(u16),
}

impl Command {
    /// Gets the command byte code.
    pub fn command_byte(&self) -> u8 {
        match self {
            Self::Next => 0x01,
            Self::Previous => 0x02,
            Self::PlayTrack(_) => 0x03,
            Self::VolumeUp => 0x04,
            Self::VolumeDown => 0x05,
            Self::SetVolume(_) => 0x06,
            Self::SetEqualizerPreset(_) => 0x07,
            Self::RepeatTrack(_) => 0x08,
            Self::SetPlaybackSource(_) => 0x09,
            Self::EnterStandby => 0x0A,
            Self::ExitStandby => 0x0B,
            Self::ResetModule => 0x0C,
            Self::Resume => 0x0D,
            Self::Pause => 0x0E,
            Self::PlayFromFolder(_, _) => 0x0F,
            Self::SetVolumeGain(_, _) => 0x10,
            Self::RepeatAll(_) => 0x11,
            Self::PlayFromMp3Folder(_) => 0x12,
        }
    }

    pub fn parameter(&self) -> Result<Parameter> {
        match *self {
            Self::SetVolume(level) => {
                if level > 30 {
                    return Err(Error::InvalidParameter(
                        "volume must be in the range 0 to 30",
                    ));
                }

                Ok(Parameter::new(0, level))
            } // 0x06
            Self::SetVolumeGain(enable, level) => {
                if level > 30 {
                    return Err(Error::InvalidParameter(
                        "volume gain must be in the range 0 to 30",
                    ));
                }

                Ok(Parameter::new(enable.into(), level))
            } // 0x10
            Self::PlayFromFolder(folder, track) => Ok(Parameter::new(folder, track)),    // 0x0F
            Self::PlayTrack(track_num) => Ok(Parameter::from(track_num)),                // 0x03
            Self::RepeatTrack(track_num) => Ok(Parameter::from(track_num)),              // 0x08
            Self::PlayFromMp3Folder(track_num) => Ok(Parameter::from(track_num)),        // 0x12
            Self::RepeatAll(enable) => Ok(Parameter::new(0, enable.into())),             // 0x11
            Self::SetEqualizerPreset(preset) => Ok(Parameter::new(0, preset as u8)),     // 0x07
            Self::SetPlaybackSource(source) => Ok(Parameter::new(0, source as u8)),      // 0x09
            Self::EnterStandby => Ok(Parameter::new(0, 1)),                              // 0x0A
            Self::ExitStandby => Ok(Parameter::new(0, 1)),                               // 0x0B
            _ => Ok(Parameter::default()),
        }
    }
}
