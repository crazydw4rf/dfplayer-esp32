use crate::{CommandPayload, CommandType, FramePayload, Result};

/// A collection of DFPlayer Mini query commands.
///
/// Query commands request current state, settings, or track counts from the module.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Query {
    /// Queries the current active playback storage source (USB, TF Card, AUX, Sleep, Flash).
    PlaybackSource,

    /// Queries the current playback status (playing, paused, stopped on an active source, or in sleep).
    PlaybackStatus,

    /// Queries the current output volume level (0–30).
    VolumeLevel,

    /// Queries the current equalizer preset (Normal, Pop, Rock, Jazz, Classic, Bass).
    EqualizerPreset,

    /// Queries the total number of audio tracks available on the USB flash drive.
    TracksCountUsb,

    /// Queries the total number of audio tracks available on the TF (MicroSD) card.
    TracksCountTf,

    /// Queries the currently playing track number on the USB flash drive.
    CurrentTrackUsb,

    /// Queries the currently playing track number on the TF (MicroSD) card.
    CurrentTrackTf,

    /// Queries the total number of audio tracks in a specific folder (Folder 1–99).
    TrackCountsFolder(u8),
}

impl CommandPayload for Query {
    const TYPE: CommandType = CommandType::Query;

    fn command_byte(self) -> u8 {
        match self {
            Self::PlaybackSource => 0x3F,
            Self::PlaybackStatus => 0x42,
            Self::VolumeLevel => 0x43,
            Self::EqualizerPreset => 0x44,
            Self::TracksCountUsb => 0x47,
            Self::TracksCountTf => 0x48,
            Self::CurrentTrackUsb => 0x4B,
            Self::CurrentTrackTf => 0x4C,
            Self::TrackCountsFolder(_) => 0x4F,
        }
    }

    fn parameter(self) -> Result<FramePayload> {
        match self {
            Self::TrackCountsFolder(folder) => Ok(FramePayload::new(0, folder)),
            _ => Ok(FramePayload::default()),
        }
    }
}
