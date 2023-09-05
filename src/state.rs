use crate::evidence::Evidence;
pub struct SelectionState {
    evidences: u32,
    difficulty: u32
}

impl SelectionState {
    pub fn new() -> SelectionState {
        Self {
            evidences: 0,
            difficulty: 3
        }
    }

    pub fn reset(self: &mut Self) {
        self.evidences = 0;
    }

    pub fn mark(self: &mut Self, evidence: Evidence) {
        self.evidences |= evidence as u32;
    }

    pub fn unmark(self: &mut Self, evidence: Evidence) {
        self.evidences &= !(evidence as u32);
    }

    pub fn marked(self: &Self, evidence: Evidence) -> bool {
        (self.evidences & (evidence as u32)) != 0
    }

    pub fn toggle(self: &mut Self, evidence: Evidence) {
        self.evidences ^= evidence as u32;
    }

    pub fn next_difficulty(self: &mut Self) {
        match self.difficulty {
            0 => self.difficulty = 3,
            1 => self.difficulty = 0,
            2 => self.difficulty = 1,
            3 => self.difficulty = 2,
            _ => self.difficulty = 3
        }
        self.reset();
    }

    pub fn current_difficulty(self: &Self) -> u32 {
        self.difficulty
    }
}