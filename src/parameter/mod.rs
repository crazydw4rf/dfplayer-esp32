pub mod play_track;

pub use play_track::*;

#[derive(Clone, Copy, Default)]
pub struct Parameter {
    param1: u8,
    param2: u8,
}

impl Parameter {
    pub(crate) fn new(param1: u8, param2: u8) -> Self {
        Self { param1, param2 }
    }
}

impl From<Parameter> for (u8, u8) {
    fn from(value: Parameter) -> Self {
        (value.param1, value.param2)
    }
}
