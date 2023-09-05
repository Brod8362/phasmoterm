use serde::Deserialize;

use crate::evidence::Evidence;

#[derive(Deserialize, Clone)]
pub struct Ghost {
    pub name: String,
    #[serde(alias = "speed")]
    pub min_speed: f32,
    #[serde(default)]
    pub max_speed: f32,
    #[serde(alias = "hunt_sanity")]
    pub min_hunt_sanity: f32,
    #[serde(default)]
    pub max_hunt_sanity: f32,
    pub evidence: Vec<Evidence>,
    pub description: String
}

impl Ghost {
    pub fn has_evidence(self: &Self, evidence: Evidence) -> bool {
        self.evidence.contains(&evidence)
    }
}