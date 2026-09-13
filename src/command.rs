use crate::parameter::{EqualizerPreset, Parameter};
use crate::{PlaybackSource, SwitchState};
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
    SetVolumeGain(SwitchState, u8),

    /// Enables or disables repeat playback for all tracks.
    RepeatAll(SwitchState),

    /// Plays a specific track (1–3000) in the MP3 folder.
    PlayFromMp3Folder(u16),

    /// Plays a specific track (1–3000) in the ADVERT folder.
    PlayFromAdvertFolder(u16),

    /// Plays a specific track from a large folder (up to 15 folders, up to 3000 tracks).
    PlayFromFolderLarge(u8, u16),

    /// Stops the currently playing advertisement and resumes previous playback.
    SkipAdvertisement,

    /// Stops all playback.
    StopAllPlayback,

    /// Plays and repeats all tracks in a specific folder (Folder 1–99).
    RepeatPlaybackFolder(u8),

    /// Starts random playback for all tracks.
    StartRandomPlayback,

    /// Enables or disables repeating the currently playing track.
    RepeatCurrentPlayback(SwitchState),

    /// Sets the DAC state (on or off).
    SetDacState(SwitchState),

    Raw(u8, u8, u8),
}

impl Command {
    /// Gets the command byte code.
    pub fn command_byte(&self) -> u8 {
        match *self {
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
            Self::PlayFromAdvertFolder(_) => 0x13,
            Self::PlayFromFolderLarge(_, _) => 0x14,
            Self::SkipAdvertisement => 0x15,
            Self::StopAllPlayback => 0x16,
            Self::RepeatPlaybackFolder(_) => 0x17,
            Self::StartRandomPlayback => 0x18,
            Self::RepeatCurrentPlayback(_) => 0x19,
            Self::SetDacState(_) => 0x1A,
            Self::Raw(cmd, _, _) => cmd,
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
            Self::SetVolumeGain(state, level) => {
                if level > 31 {
                    return Err(Error::InvalidParameter(
                        "volume gain must be in the range 0 to 31",
                    ));
                }

                Ok(Parameter::new(state.into(), level))
            } // 0x10
            Self::PlayFromFolderLarge(folder, track_num) => {
                if !(1..=15).contains(&folder) || !(1..=3000).contains(&track_num) {
                    return Err(Error::InvalidParameter(
                        "folder must be 1–15 and track number must be 1–3000",
                    ));
                }

                let params: u16 = ((folder as u16) << 12) | (track_num & 0x0FFF);

                Ok(Parameter::from(params))
            } // 0x14
            Self::RepeatPlaybackFolder(folder) => {
                if !(1..=99).contains(&folder) {
                    return Err(Error::InvalidParameter(
                        "folder must be in the range 1 to 99",
                    ));
                }

                Ok(Parameter::new(0, folder))
            } // 0x17
            Self::PlayFromFolder(folder, track_num) => Ok(Parameter::new(folder, track_num)), // 0x0F
            Self::PlayTrack(track_num) => Ok(Parameter::from(track_num)), // 0x03
            Self::RepeatTrack(track_num) => Ok(Parameter::from(track_num)), // 0x08
            Self::PlayFromMp3Folder(track_num) => Ok(Parameter::from(track_num)), // 0x12
            Self::PlayFromAdvertFolder(track_num) => Ok(Parameter::from(track_num)), // 0x13
            Self::RepeatAll(state) => Ok(Parameter::new(0, state.into())), // 0x11
            Self::SetEqualizerPreset(preset) => Ok(Parameter::new(0, preset as u8)), // 0x07
            Self::SetPlaybackSource(source) => Ok(Parameter::new(0, source as u8)), // 0x09
            Self::EnterStandby => Ok(Parameter::new(0, 1)),               // 0x0A
            Self::ExitStandby => Ok(Parameter::new(0, 1)),                // 0x0B
            Self::RepeatCurrentPlayback(state) => Ok(Parameter::new(0, state.invert().into())), // 0x19
            Self::SetDacState(state) => Ok(Parameter::new(0, state.invert().into())), // 0x1A
            Self::Raw(_, param1, param2) => Ok(Parameter::new(param1, param2)),
            _ => Ok(Parameter::default()),
        }
    }
}
