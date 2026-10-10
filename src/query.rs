use crate::{FramePayload, Result, error};

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
    Volume,

    /// Queries the current equalizer preset (Normal, Pop, Rock, Jazz, Classical, Bass).
    EqualizerPreset,

    /// Queries the total number of audio tracks available on the USB flash drive.
    TrackCountUsb,

    /// Queries the total number of audio tracks available on the TF (MicroSD) card.
    TrackCountTf,

    /// Queries the currently playing track number on the USB flash drive.
    CurrentTrackUsb,

    /// Queries the currently playing track number on the TF (MicroSD) card.
    CurrentTrackTf,

    /// Queries the total number of audio tracks in a specific folder (Folder 1–99).
    TrackCountFolder(u8),
}

impl TryFrom<Query> for FramePayload {
    type Error = error::Error;

    fn try_from(value: Query) -> Result<Self, Self::Error> {
        match value {
            Query::PlaybackSource => Ok(FramePayload::from_cmd(0x3F)),
            Query::PlaybackStatus => Ok(FramePayload::from_cmd(0x42)),
            Query::Volume => Ok(FramePayload::from_cmd(0x43)),
            Query::EqualizerPreset => Ok(FramePayload::from_cmd(0x44)),
            Query::TrackCountUsb => Ok(FramePayload::from_cmd(0x47)),
            Query::TrackCountTf => Ok(FramePayload::from_cmd(0x48)),
            Query::CurrentTrackUsb => Ok(FramePayload::from_cmd(0x4B)),
            Query::CurrentTrackTf => Ok(FramePayload::from_cmd(0x4C)),
            Query::TrackCountFolder(folder) => Ok(FramePayload::new(0x4F, folder as u16)),
        }
    }
}
