use ratatui::style::Color;

pub fn hunt_sanity_color(sanity: u32) -> Color {
    if sanity < 50 {
        Color::LightGreen
    } else if sanity > 50 {
        Color::LightRed
    } else {
        Color::White
    }
}