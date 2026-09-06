use crate::parameter::{EqualizerPreset, Parameter, PlaybackMode};
use crate::{Result, error::Error};

/// A collection of DFPlayer Mini commands.
#[derive(Clone, Copy)]
pub enum Command {
    /// Plays the next track.
    Next,

    /// Plays the previous track.
    Previous,

    /// Plays a specific track number (1–2999).
    PlayTrack(u16),

    /// Increases the volume by one step.
    VolumeUp,

    /// Decreases the volume by one step.
    VolumeDown,

    /// Sets the volume level (0–30).
    SetVolume(u8),

    /// Sets the equalizer preset (Normal, Pop, Rock, Jazz, Classic, Bass).
    SetEqualizerPreset(EqualizerPreset),

    /// Sets the playback mode (Repeat, Folder repeat, Single repeat, Random).
    SetPlaybackMode(PlaybackMode),

    /// Sets the playback source (USB, TF Card, AUX, Sleep, Flash).
    SetPlaybackSource,

    /// Enters low-power standby mode.
    EnterStandby,

    /// Puts the module in normal working mode (exits standby).
    ExitStandby,

    /// Resets the DFPlayer Mini module.
    ResetModule,

    /// Resumes playback.
    PlaybackResume,

    /// Pauses playback.
    PlaybackPause,

    /// Plays a specific track from a specified folder (Folder 1–99, File 1–255).
    SetPlaybackFolder,

    /// Sets the volume gain (gain 0–31).
    SetVolumeGain,

    /// Enables (1) or stops (0) repeat playback.
    RepeatPlay,
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
            Self::SetPlaybackMode(_) => 0x08,
            Self::SetPlaybackSource => 0x09,
            Self::EnterStandby => 0x0A,
            Self::ExitStandby => 0x0B,
            Self::ResetModule => 0x0C,
            Self::PlaybackResume => 0x0D,
            Self::PlaybackPause => 0x0E,
            Self::SetPlaybackFolder => 0x0F,
            Self::SetVolumeGain => 0x10,
            Self::RepeatPlay => 0x11,
        }
    }

    pub fn parameter(&self) -> Result<Parameter> {
        match *self {
            Self::Next => Ok(Parameter::default()),
            Self::Previous => Ok(Parameter::default()),
            Self::PlayTrack(track_num) => {
                if !(1..=2999).contains(&track_num) {
                    return Err(Error::InvalidParameter(
                        "track number must be between 1 and 2999".into(),
                    ));
                }

                Ok(Parameter::from(track_num))
            }
            Self::VolumeUp => Ok(Parameter::default()),
            Self::VolumeDown => Ok(Parameter::default()),
            Self::SetVolume(level) => {
                if !(0..=30).contains(&level) {
                    return Err(Error::InvalidParameter(
                        "volume must be in the range 0 to 30".into(),
                    ));
                }

                Ok(Parameter::new(0, level))
            }
            Self::SetEqualizerPreset(preset) => Ok(Parameter::new(0, preset as u8)),
            Self::SetPlaybackMode(mode) => Ok(Parameter::new(0, mode as u8)),
            Self::SetPlaybackSource => Ok(Parameter::default()),
            Self::EnterStandby => Ok(Parameter::default()),
            Self::ExitStandby => Ok(Parameter::default()),
            Self::ResetModule => Ok(Parameter::default()),
            Self::PlaybackResume => Ok(Parameter::default()),
            Self::PlaybackPause => Ok(Parameter::default()),
            Self::SetPlaybackFolder => Ok(Parameter::default()),
            Self::SetVolumeGain => Ok(Parameter::default()),
            Self::RepeatPlay => Ok(Parameter::default()),
        }
    }
}
