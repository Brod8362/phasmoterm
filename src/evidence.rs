use ratatui::style::Color;
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

    pub fn color(self: &Self) -> Color {
        match self {
            Evidence::EMF => Color::Red,
            Evidence::DOTS => Color::Green,
            Evidence::Ultraviolet => Color::Magenta,
            Evidence::Freezing => Color::LightCyan,
            Evidence::GhostOrbs => Color::Yellow,
            Evidence::Writing => Color::Blue,
            Evidence::SpiritBox => Color::Rgb(215, 95, 0),
        }
    }

    pub fn name(self: &Self) -> &str {
        match self {
            Evidence::EMF => "EMF 5",
            Evidence::DOTS => "DOTS Projector",
            Evidence::Ultraviolet => "Ultraviolet",
            Evidence::Freezing => "Freezing Temperatures",
            Evidence::GhostOrbs => "Ghost Orbs",
            Evidence::Writing => "Ghost Writing",
            Evidence::SpiritBox => "Spirit Box"
        }
    }

    pub fn symbol(self: &Self) -> &str {
        match self {
            Evidence::EMF => "EMF",
            Evidence::DOTS => "DTS",
            Evidence::Ultraviolet => "UV",
            Evidence::Freezing => "FZ",
            Evidence::GhostOrbs => "ORB",
            Evidence::Writing => "WR",
            Evidence::SpiritBox => "BOX"
        }
    }
}