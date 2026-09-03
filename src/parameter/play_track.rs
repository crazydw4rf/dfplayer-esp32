use crate::Parameter;
use crate::{Result, error::Error};

pub enum PlayTrackParam {
    /// Play the first track in the root folder.
    FirstTrack,
    /// Play track 2999, the maximum supported track number, in the root folder.
    MaxTrack,
    /// Play the specified track number in the root folder.
    TrackNumber(u16),
}

impl TryFrom<PlayTrackParam> for Parameter {
    type Error = Error;

    fn try_from(value: PlayTrackParam) -> Result<Self> {
        match value {
            PlayTrackParam::FirstTrack => Ok(Self::new(0x0, 0x01)),
            PlayTrackParam::MaxTrack => Ok(Self::new(0x0B, 0xB7)), // 2999
            PlayTrackParam::TrackNumber(track_num) => {
                if track_num >= 3000 {
                    return Err(Error::InvalidParameter);
                }

                Ok(Self::new((track_num >> 8) as u8, (track_num & 0xFF) as u8))
            }
        }
    }
}
