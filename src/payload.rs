use derive_more::TryFrom;

#[derive(Clone, Copy, Default, Debug)]
pub struct FramePayload {
    msb: u8,
    lsb: u8,
}

impl FramePayload {
    pub fn new(msb: u8, lsb: u8) -> Self {
        Self { msb, lsb }
    }

    pub fn as_u16(&self) -> u16 {
        u16::from_be_bytes([self.msb, self.lsb])
    }
}

impl From<FramePayload> for (u8, u8) {
    fn from(value: FramePayload) -> Self {
        (value.msb, value.lsb)
    }
}

impl From<u16> for FramePayload {
    fn from(value: u16) -> Self {
        let [param1, param2] = u16::to_be_bytes(value);
        Self::new(param1, param2)
    }
}

impl From<[u8; 10]> for FramePayload {
    fn from(frame: [u8; 10]) -> Self {
        Self::new(frame[5], frame[6])
    }
}

/// The equalizer (EQ) audio presets supported by the DFPlayer Mini.
#[derive(TryFrom, Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
#[try_from(repr)]
pub enum EqualizerPreset {
    /// Normal (flat) equalizer setting.
    Normal = 0x00,
    /// Pop music equalizer preset.
    Pop = 0x01,
    /// Rock music equalizer preset.
    Rock = 0x02,
    /// Jazz music equalizer preset.
    Jazz = 0x03,
    /// Classical music equalizer preset.
    Classic = 0x04,
    /// Bass boost equalizer preset.
    Bass = 0x05,
}

/// The storage sources supported by the DFPlayer Mini for audio playback.
#[derive(TryFrom, Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
#[try_from(repr)]
pub enum PlaybackSource {
    /// USB flash drive.
    Usb = 0x01,
    /// MicroSD / TF card.
    Tf = 0x02,
    /// AUX input.
    Aux = 0x03,
    /// Sleep mode (no active playback source).
    Sleep = 0x04,
    /// On-board NOR flash storage.
    Flash = 0x05,
}

/// The current playback status of the DFPlayer Mini module.
///
/// Indicates whether the module is currently playing, paused, stopped on an active
/// storage source, or in low-power sleep mode.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PlaybackStatus {
    /// A track is currently being played on the specified storage source.
    Playing(PlaybackSource),
    /// Playback is currently paused on the specified storage source.
    Paused(PlaybackSource),
    /// Playback is stopped on the specified storage source.
    Stopped(PlaybackSource),
    /// The module is in low-power sleep mode.
    Sleep,
}

macro_rules! impl_try_from_frame_payload {
    ($($type:ty),+ $(,)?) => {
        $(
            impl TryFrom<FramePayload> for $type {
                type Error = crate::error::Error;

                fn try_from(payload: FramePayload) -> Result<Self, Self::Error> {
                    Ok(Self::try_from(payload.lsb)?)
                }
            }
        )+
    };
}

impl_try_from_frame_payload!(EqualizerPreset, PlaybackSource);

impl TryFrom<FramePayload> for PlaybackStatus {
    type Error = crate::error::Error;

    fn try_from(payload: FramePayload) -> Result<Self, Self::Error> {
        if payload.msb == 0x10 {
            return Ok(Self::Sleep);
        }

        let source = PlaybackSource::try_from(payload.msb)?;
        match payload.lsb {
            0x00 => Ok(Self::Stopped(source)),
            0x01 => Ok(Self::Playing(source)),
            0x02 => Ok(Self::Paused(source)),
            _ => Err(crate::error::Error::InvalidParameter(
                "invalid playback state received in status query",
            )),
        }
    }
}

impl TryFrom<FramePayload> for u8 {
    type Error = crate::error::Error;

    fn try_from(payload: FramePayload) -> Result<Self, Self::Error> {
        Ok(payload.lsb)
    }
}

impl TryFrom<FramePayload> for u16 {
    type Error = crate::error::Error;

    fn try_from(payload: FramePayload) -> Result<Self, Self::Error> {
        Ok(payload.as_u16())
    }
}

impl TryFrom<FramePayload> for () {
    type Error = crate::error::Error;

    fn try_from(_: FramePayload) -> Result<Self, Self::Error> {
        Ok(())
    }
}

/// Represents a two-state switch (On or Off) used for toggling hardware features.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SwitchState {
    /// State is enabled / active / on.
    On = 0x1,
    /// State is disabled / inactive / off.
    Off = 0x0,
}

impl SwitchState {
    pub fn invert(self) -> Self {
        match self {
            Self::On => Self::Off,
            Self::Off => Self::On,
        }
    }
}

impl From<SwitchState> for u8 {
    fn from(value: SwitchState) -> Self {
        value as u8
    }
}
