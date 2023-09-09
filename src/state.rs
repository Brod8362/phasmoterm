use crate::{evidence::Evidence, ghosts::Ghost};

#[derive(Copy, Clone, PartialEq)]
pub enum MarkState {
    Neutral,
    Positive,
    Negative
}

pub struct SelectionState {
    evidences: [MarkState; 7],
    omitted_ghosts: Vec<String>,
    difficulty: u32,
    smudge_timer: f32
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
            difficulty: 3,
            smudge_timer: 0f32,
            omitted_ghosts: Vec::new()
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
        self.smudge_timer = 0.0f32;
        self.omitted_ghosts.clear();
    }

    pub fn selected_count_raw(self: &Self) -> usize {
        self.evidences.into_iter().filter(|k| k == &MarkState::Positive).count()
    }

    pub fn selected_count(self: &Self) -> usize {
        let mut c: u32 = self.selected_count_raw() as u32;
        if self.mimic_possible() && self.evidences[Evidence::GhostOrbs as usize] == MarkState::Positive {
            c = c-1; //if a mimic is possible then don't count ghost orbs as a selected item
        }
        u32::max(0, c) as usize
    }

    pub fn marked(self: &Self, evidence: Evidence) -> MarkState {
        self.evidences[evidence as usize]
    }

    fn mimic_possible(self: &Self) -> bool {
        // this exists for mimic handling logic
        // mimic wlil never show DOTS, writing, or EMF5
        if self.marked(Evidence::DOTS) == MarkState::Positive 
            || self.marked(Evidence::Writing) == MarkState::Positive
            || self.marked(Evidence::EMF) == MarkState::Positive {
                return false;
            }
        true
    }

    pub fn toggle(self: &mut Self, evidence: Evidence) {
        match self.evidences[evidence as usize] {
            MarkState::Neutral => {
                if self.mimic_possible() && evidence == Evidence::GhostOrbs { //always allow marking mimic if its possible
                    self.evidences[evidence as usize] = MarkState::Positive;
                } else if self.selected_count() < self.difficulty as usize {
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

    pub fn possible_ghosts<'a>(self: &'a Self, ghosts: &'a Vec<Ghost>) -> Vec<&'a Ghost> {
        //exception for mimic 
        if self.selected_count_raw() > self.difficulty as usize && self.mimic_possible() {
            //only mimic is possible 
            return ghosts.iter().filter(|k| k.name == "The Mimic").collect();
        }
        let valid: Vec<&Ghost> = ghosts.iter().filter(|ghost| {
            //check all evidences against the mark state
            for e in crate::evidence::ALL {
                let mark_state = self.evidences[e as usize];
                if mark_state == MarkState::Positive {
                    if !ghost.has_evidence(e) {
                        return false;
                    }
                }
                if mark_state == MarkState::Negative { //when marked a missing
                    if self.difficulty == 3 {
                        if ghost.has_evidence(e) {
                            return false;
                        }
                    } else if self.difficulty > 0 { //only rule out guaranteed evidences
                        if ghost.is_guaranteed_evidence(e) {
                            return false;
                        }
                    }
                    
                }
            }
            true
        }).collect();
        valid
    }
    
    // Difficulty handling

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

    //Smudge timer

    pub fn start_smudge(self: &mut Self) {
        self.smudge_timer = 180f32;
    }

    pub fn tick_smudge(self: &mut Self, delta: f32) {
        self.smudge_timer = f32::max(0f32, self.smudge_timer - delta);
    }
    
    pub fn smudge_remaining(self: &Self) -> u32 {
        f32::round(self.smudge_timer) as u32
    }

    // omit

    pub fn toggle_omit<'a>(self: &'a mut Self, ghost: &'a Ghost) {
        if self.is_omitted(ghost) {
            //remove from omitted list
            self.omitted_ghosts.retain(|g| *g != ghost.name);
        } else {
            self.omitted_ghosts.push(ghost.name.clone());
        }
    }

    pub fn is_omitted(self: &Self, ghost: &Ghost) -> bool {
        self.omitted_ghosts.iter().any(|g| g == &ghost.name)
    }
}