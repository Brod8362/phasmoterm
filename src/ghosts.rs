use crate::evidence::Evidence;

pub struct Ghost {
    pub name: String,
    pub min_speed: f32,
    pub max_speed: f32,
    pub min_hunt_sanity: f32,
    pub max_hunt_sanity: f32,
    evidence: [Evidence; 3]
}

impl Ghost {
    pub fn has_evidence(self: &Self, evidence: Evidence) -> bool{
        self.evidence.contains(&evidence)
    }
}