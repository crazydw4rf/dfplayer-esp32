#[derive(Clone, Copy, Default, Debug)]
pub struct Parameter {
    param1: u8,
    param2: u8,
}

impl Parameter {
    pub fn new(param1: u8, param2: u8) -> Self {
        Self { param1, param2 }
    }
}

impl From<Parameter> for (u8, u8) {
    fn from(value: Parameter) -> Self {
        (value.param1, value.param2)
    }
}

impl From<u16> for Parameter {
    fn from(value: u16) -> Self {
        let [param1, param2] = u16::to_be_bytes(value);
        Self::new(param1, param2)
    }
}

#[derive(Clone, Copy, Debug)]
#[repr(u8)]
pub enum EqualizerPreset {
    Normal = 0x00,
    Pop = 0x01,
    Rock = 0x02,
    Jazz = 0x03,
    Classic = 0x04,
    Bass = 0x05,
}

#[derive(Clone, Copy, Debug)]
#[repr(u8)]
pub enum PlaybackSource {
    Usb = 0x01,
    Tf = 0x02,
    Aux = 0x03,
    Sleep = 0x04,
    Flash = 0x05,
}

#[derive(Clone, Copy, Debug)]
#[repr(u8)]
pub enum SwitchState {
    On = 0x1,
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
