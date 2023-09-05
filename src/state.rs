use crate::evidence::Evidence;
pub struct SelectionState {
    evidences: i32,
    difficulty: i32
}

impl SelectionState {
    fn new() -> SelectionState {
        Self {
            evidences: 0,
            difficulty: 0
        }
    }

    fn reset(self: &mut Self) {
        self.evidences = 0;
    }

    fn mark(self: &mut Self, evidence: Evidence) {
        self.evidences = self.evidences | (evidence as i32);
    }

    fn unmark(self: &mut Self, evidence: Evidence) {
        self.evidences = self.evidences & !(evidence as i32);
    }
}