use crate::error::Error::InvalidParameter;
use crate::payload::{EqualizerPreset, FramePayload, PlaybackSource, SwitchState};
use crate::{ParamLsb, ParamMsb, Result};

/// A collection of DFPlayer Mini commands.
#[derive(Clone, Copy, Debug)]
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
    Reset,

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
    PlayLargeFolder(u8, u16),

    /// Stops the currently playing advertisement and resumes previous playback.
    SkipAdvertisement,

    /// Stops all playback.
    Stop,

    /// Plays and repeats all tracks in a specific folder (Folder 1–99).
    RepeatFolder(u8),

    /// Starts random playback for all tracks.
    PlayRandom,

    /// Enables or disables repeating the currently playing track.
    RepeatCurrentTrack(SwitchState),

    /// Sets the DAC state (on or off).
    SetDac(SwitchState),

    /// Sends a raw command with a custom command byte, MSB parameter, and LSB parameter.
    Raw(u8, ParamMsb, ParamLsb),
}

impl Command {
    fn play_track(track_num: u16) -> Result<FramePayload> {
        if track_num == 0 {
            return Err(InvalidParameter(
                "track number cannot be 0 (must be 1–3000)",
            ));
        }

        Ok(FramePayload::new(0x03, track_num))
    }

    fn set_volume(volume: u8) -> Result<FramePayload> {
        if volume > 30 {
            return Err(InvalidParameter(
                "volume level cannot exceed 30 (range: 0–30)",
            ));
        }

        Ok(FramePayload::new(0x06, volume.into()))
    }

    fn play_from_folder(folder: u8, track: u8) -> Result<FramePayload> {
        if !(1..=99).contains(&folder) || track == 0 {
            return Err(InvalidParameter(
                "folder must be between 1 and 99, and track number cannot be 0",
            ));
        }

        Ok(FramePayload::new(0x0F, u16::from_be_bytes([folder, track])))
    }

    fn set_volume_gain(state: SwitchState, volume: u8) -> Result<FramePayload> {
        if volume > 31 {
            return Err(InvalidParameter("volume gain must be between 0 and 31"));
        }

        Ok(FramePayload::new(
            0x10,
            u16::from_be_bytes([state as u8, volume]),
        ))
    }

    fn play_large_folder(folder: u8, track_num: u16) -> Result<FramePayload> {
        if !(1..=15).contains(&folder) || !(1..=3000).contains(&track_num) {
            return Err(InvalidParameter(
                "folder must be between 1 and 15, and track number must be between 1 and 3000",
            ));
        }

        let params: u16 = ((folder as u16) << 12) | (track_num & 0x0FFF);

        Ok(FramePayload::new(0x14, params))
    }

    fn repeat_folder(folder: u8) -> Result<FramePayload> {
        if !(1..=99).contains(&folder) {
            return Err(InvalidParameter("folder must be between 1 and 99"));
        }

        Ok(FramePayload::new(0x17, folder as u16))
    }
}

impl TryFrom<Command> for FramePayload {
    type Error = crate::error::Error;

    fn try_from(cmd: Command) -> Result<Self, Self::Error> {
        match cmd {
            Command::Next => Ok(FramePayload::from_cmd(0x01)),
            Command::Previous => Ok(FramePayload::from_cmd(0x02)),
            Command::PlayTrack(track_num) => Command::play_track(track_num),
            Command::VolumeUp => Ok(FramePayload::from_cmd(0x04)),
            Command::VolumeDown => Ok(FramePayload::from_cmd(0x05)),
            Command::SetVolume(volume) => Command::set_volume(volume),
            Command::SetEqualizerPreset(preset) => Ok(FramePayload::new(0x07, preset as u16)),
            Command::RepeatTrack(track_num) => Ok(FramePayload::new(0x08, track_num)),
            Command::SetPlaybackSource(source) => Ok(FramePayload::new(0x09, source as u16)),
            Command::EnterStandby => Ok(FramePayload::from_cmd(0x0A)),
            Command::ExitStandby => Ok(FramePayload::from_cmd(0x0B)),
            Command::Reset => Ok(FramePayload::from_cmd(0x0C)),
            Command::Resume => Ok(FramePayload::from_cmd(0x0D)),
            Command::Pause => Ok(FramePayload::from_cmd(0x0E)),
            Command::PlayFromFolder(folder, track) => Command::play_from_folder(folder, track),
            Command::SetVolumeGain(state, volume) => Command::set_volume_gain(state, volume),
            Command::RepeatAll(state) => Ok(FramePayload::new(0x11, state as u16)),
            Command::PlayFromMp3Folder(track_num) => Ok(FramePayload::new(0x12, track_num)),
            Command::PlayFromAdvertFolder(track_num) => Ok(FramePayload::new(0x13, track_num)),
            Command::PlayLargeFolder(folder, track) => Command::play_large_folder(folder, track),
            Command::SkipAdvertisement => Ok(FramePayload::from_cmd(0x15)),
            Command::Stop => Ok(FramePayload::from_cmd(0x16)),
            Command::RepeatFolder(folder) => Command::repeat_folder(folder),
            Command::PlayRandom => Ok(FramePayload::from_cmd(0x18)),
            Command::RepeatCurrentTrack(state) => {
                Ok(FramePayload::new(0x19, state.toggle() as u16))
            }
            Command::SetDac(state) => Ok(FramePayload::new(0x1A, state.toggle() as u16)),
            Command::Raw(cmd, param1, param2) => Ok(FramePayload::new(
                cmd,
                u16::from_be_bytes([*param1, *param2]),
            )),
        }
    }
}
