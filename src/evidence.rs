use serde::Deserialize;

use crate::ghosts::Ghost;

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

impl Evidence {
    pub fn possible(self: &Self, possible_ghosts: &Vec<&Ghost>) -> bool {
        possible_ghosts.iter().find(|g| g.has_evidence(self.clone())).is_some()
    }
}