use derive_more::{Deref, From, TryFrom};

use crate::{Result, error::DeviceError};

const START_BYTE: u8 = 0x7E;
const END_BYTE: u8 = 0xEF;
const VERSION_BYTE: u8 = 0xFF;
const COMMAND_LENGTH: u8 = 0x06;
const RESPONSE_ERROR: u8 = 0x40;

#[derive(Clone, Copy, Default, Debug)]
pub struct FramePayload {
    cmd_byte: u8,
    params: u16,
}

impl FramePayload {
    pub fn new(cmd_byte: u8, params: u16) -> Self {
        Self { cmd_byte, params }
    }

    pub fn from_cmd(cmd_byte: u8) -> Self {
        Self {
            cmd_byte,
            ..Default::default()
        }
    }

    pub fn split_params(&self) -> (ParamMsb, ParamLsb) {
        let [param_msb, param_lsb] = u16::to_be_bytes(self.params);
        (ParamMsb(param_msb), ParamLsb(param_lsb))
    }

    pub fn check_error(&self) -> Result<()> {
        if self.cmd_byte == RESPONSE_ERROR {
            let error = DeviceError::try_from(self.params as u8).unwrap_or_default();
            return Err(crate::error::Error::Device(error));
        }

        Ok(())
    }

    pub fn into_send_frame(self) -> [u8; 10] {
        let (param_msb, param_lsb) = self.split_params();

        let [checksum_high, checksum_low] = (VERSION_BYTE as u16
            + COMMAND_LENGTH as u16
            + self.cmd_byte as u16
            + *param_msb as u16
            + *param_lsb as u16)
            .wrapping_neg()
            .to_be_bytes();

        [
            START_BYTE,
            VERSION_BYTE,
            COMMAND_LENGTH,
            self.cmd_byte,
            0_u8,
            *param_msb,
            *param_lsb,
            checksum_high,
            checksum_low,
            END_BYTE,
        ]
    }
}

impl From<u16> for FramePayload {
    fn from(value: u16) -> Self {
        Self::new(0, value)
    }
}

impl TryFrom<FramePayload> for u16 {
    type Error = crate::error::Error;

    fn try_from(value: FramePayload) -> Result<Self, Self::Error> {
        Ok(value.params)
    }
}

impl TryFrom<FramePayload> for u8 {
    type Error = crate::error::Error;

    fn try_from(value: FramePayload) -> Result<Self, Self::Error> {
        Ok(value.params as u8)
    }
}

impl From<[u8; 10]> for FramePayload {
    fn from(frame: [u8; 10]) -> Self {
        let params = u16::from_be_bytes([frame[5], frame[6]]);
        Self::new(frame[3], params)
    }
}

#[derive(Debug, Clone, Copy, Deref, From)]
pub struct ParamMsb(pub u8);

#[derive(Debug, Clone, Copy, Deref, From)]
pub struct ParamLsb(pub u8);

impl PartialEq<u8> for ParamMsb {
    fn eq(&self, other: &u8) -> bool {
        self.0 == *other
    }
}

impl PartialEq<u8> for ParamLsb {
    fn eq(&self, other: &u8) -> bool {
        self.0 == *other
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

impl TryFrom<FramePayload> for EqualizerPreset {
    type Error = crate::error::Error;

    fn try_from(payload: FramePayload) -> Result<Self, Self::Error> {
        Ok(Self::try_from(payload.params as u8)?)
    }
}

impl TryFrom<FramePayload> for PlaybackSource {
    type Error = crate::error::Error;

    fn try_from(payload: FramePayload) -> Result<Self, Self::Error> {
        Ok(Self::try_from(payload.params as u8)?)
    }
}

impl TryFrom<FramePayload> for PlaybackStatus {
    type Error = crate::error::Error;

    fn try_from(payload: FramePayload) -> Result<Self, Self::Error> {
        let (param_msb, param_lsb) = payload.split_params();

        if param_msb == 0x10 {
            return Ok(Self::Sleep);
        }

        let source = PlaybackSource::try_from(*param_msb)?;
        match *param_lsb {
            0x00 => Ok(Self::Stopped(source)),
            0x01 => Ok(Self::Playing(source)),
            0x02 => Ok(Self::Paused(source)),
            _ => Err(crate::error::Error::InvalidParameter(
                "invalid playback state received in status query",
            )),
        }
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
    /// Toggles the switch state (`On` -> `Off`, `Off` -> `On`).
    pub fn toggle(self) -> Self {
        match self {
            Self::On => Self::Off,
            Self::Off => Self::On,
        }
    }
}
