use serde::Deserialize;

use crate::evidence::Evidence;

#[derive(Deserialize, Clone)]
pub struct Ghost {
    pub name: String,
    pub min_speed: f32,
    #[serde(default)]
    pub max_speed: f32,
    pub min_hunt_sanity: f32,
    #[serde(default)]
    pub max_hunt_sanity: f32,
    pub evidence: [Evidence; 3],
    pub description: String
}

impl Ghost {
    pub fn has_evidence(self: &Self, evidence: Evidence) -> bool {
        self.evidence.contains(&evidence)
    }
}