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
pub enum PlaybackMode {
    Repeat = 0x00,
    FolderRepeat = 0x01,
    SingleRepeat = 0x02,
    Random = 0x03,
}
