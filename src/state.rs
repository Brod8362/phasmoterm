use crate::evidence::Evidence;

#[derive(Copy, Clone, PartialEq)]
pub enum MarkState {
    Neutral,
    Positive,
    Negative
}

pub struct SelectionState {
    evidences: [MarkState; 7],
    difficulty: u32
}

impl SelectionState {
    pub fn new() -> SelectionState {
        let states = [
            MarkState::Neutral,
            MarkState::Neutral,
            MarkState::Neutral,
            MarkState::Neutral,
            MarkState::Neutral,
            MarkState::Neutral,
            MarkState::Neutral,
        ];
        Self {
            evidences: states,
            difficulty: 3
        }
    }

    pub fn reset(self: &mut Self) {
        self.evidences[Evidence::EMF as usize] = MarkState::Neutral;
        self.evidences[Evidence::DOTS as usize] = MarkState::Neutral;
        self.evidences[Evidence::Ultraviolet as usize] = MarkState::Neutral;
        self.evidences[Evidence::Freezing as usize] = MarkState::Neutral;
        self.evidences[Evidence::GhostOrbs as usize] = MarkState::Neutral;
        self.evidences[Evidence::Writing as usize] = MarkState::Neutral;
        self.evidences[Evidence::SpiritBox as usize] = MarkState::Neutral;
    }

    pub fn selected_count(self: &Self) -> usize {
        self.evidences.into_iter().filter(|k| k == &MarkState::Positive).count()
    }

    pub fn marked(self: &Self, evidence: Evidence) -> MarkState {
        self.evidences[evidence as usize]
    }

    pub fn possible(self: &Self, evidence: Evidence) -> bool {
        //TODO
        self.evidences[evidence as usize] == MarkState::Positive
    }

    pub fn toggle(self: &mut Self, evidence: Evidence) {
        match self.evidences[evidence as usize] {
            MarkState::Neutral => {
                if self.selected_count() < self.difficulty as usize {
                    self.evidences[evidence as usize] = MarkState::Positive;
                } else {
                    self.evidences[evidence as usize] = MarkState::Negative;
                }
            },
            MarkState::Positive => {
                self.evidences[evidence as usize] = MarkState::Negative;
            },
            MarkState::Negative => {
                self.evidences[evidence as usize] = MarkState::Neutral;
            }
        }
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