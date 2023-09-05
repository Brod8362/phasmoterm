use serde::Deserialize;

#[repr(usize)]
#[derive(Copy, Clone, PartialEq, Deserialize)]
pub enum Evidence {
    EMF = 0,
    DOTS = 1,
    Ultraviolet = 2,
    Freezing = 3,
    GhostOrbs = 4,
    Writing = 5,
    SpiritBox = 6
}