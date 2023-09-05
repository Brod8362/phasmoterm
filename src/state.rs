use crate::evidence::Evidence;
pub struct SelectionState {
    evidences: i32,
    difficulty: i32
}

impl SelectionState {
    pub fn new() -> SelectionState {
        Self {
            evidences: 0,
            difficulty: 0
        }
    }

    pub fn reset(self: &mut Self) {
        self.evidences = 0;
    }

    pub fn mark(self: &mut Self, evidence: Evidence) {
        self.evidences |= (evidence as i32);
    }

    pub fn unmark(self: &mut Self, evidence: Evidence) {
        self.evidences &= !(evidence as i32);
    }

    pub fn marked(self: &Self, evidence: Evidence) -> bool {
        (self.evidences & (evidence as i32)) != 0
    }

    pub fn toggle(self: &mut Self, evidence: Evidence) {
        self.evidences ^= evidence as i32;
    }
}