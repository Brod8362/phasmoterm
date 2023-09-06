use ratatui::{widgets::Paragraph, text::{Line, Span}, style::Stylize};
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
    pub min_hunt_sanity: u32,
    #[serde(default)]
    pub max_hunt_sanity: u32,
    pub evidence: Vec<Evidence>,
    #[serde(default)]
    pub guaranteed: Vec<Evidence>, //evidence that is guaranteed on nightmare/insanity
    pub description: String
}

impl Ghost {
    pub fn has_evidence(self: &Self, evidence: Evidence) -> bool {
        self.evidence.contains(&evidence)
    }

    pub fn is_guaranteed_evidence(self: &Self, evidence: Evidence) -> bool {
        self.guaranteed.contains(&evidence)
    }

    pub fn render_information(self: &Self) -> Paragraph {
        let speed_line = if self.max_speed != 0.0f32 {
            Line::from(
                vec![
                    "Speed: ".bold(), 
                    Span::from(format!("{}m/s", self.min_speed)),
                    Span::from(" - "),
                    Span::from(format!("{}m/s", self.max_speed))
                ]
            )
        } else {
            Line::from(
                vec![
                    "Speed: ".bold(), 
                    Span::from(format!("{}m/s", self.min_speed))
                ]
            )
        };

        let hunt_line = if self.max_hunt_sanity != 0 {
            Line::from(
                vec![
                    "Hunt Sanity: ".bold(), 
                    Span::from(format!("{}%", self.min_hunt_sanity)),
                    Span::from(" - "),
                    Span::from(format!("{}%", self.max_hunt_sanity))
                ]
            )
        } else {
            Line::from(vec!["Hunt Sanity: ".bold(), Span::from(format!("{}%", self.min_hunt_sanity))])
        };

        let mut evidence_line: Vec<Span> = Vec::new();
        evidence_line.push("Evidence: ".bold());
        for e in &self.evidence {
            let mut string = String::from(e.name());
            string.push(' ');
            let span = Span::from(string).bold().fg(e.color());
            evidence_line.push(span);
        }

        let mut lines = vec![
            Line::from(vec!["Name: ".bold(), Span::from(self.name.clone())]),
            speed_line,
            hunt_line,
            Line::from(evidence_line)
        ];
        
        let mut desc: Vec<Line> = self.description.split('\n').map(|l| Line::from(l)).collect();
        lines.append(&mut desc);
        //split description into multiple line objects
        Paragraph::new(lines)
    }
}