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

pub const ALL: [Evidence; 7] = [
    Evidence::EMF,
    Evidence::DOTS,
    Evidence::Ultraviolet,
    Evidence::Freezing,
    Evidence::GhostOrbs,
    Evidence::Writing,
    Evidence::SpiritBox
];