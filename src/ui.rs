use ratatui::style::Color;

pub struct UI {
    pub muted: Color,
    pub accent: Color,
    pub footer_bg: Color,
}

impl UI {
    pub fn default() -> Self {
        Self {
            muted: Color::DarkGray,
            accent: Color::Cyan,
            footer_bg: Color::Black,
        }
    }
}
