use ratatui::style::Color;

pub struct UI {
    pub muted: Color,
    pub accent: Color,
    pub footer_bg: Color,
    pub success: Color,
    pub warn: Color,
    pub danger: Color,
    pub gauge_bg: Color,
}

impl UI {
    pub fn default() -> Self {
        Self {
            muted: Color::DarkGray,
            accent: Color::Cyan,
            footer_bg: Color::Black,
            success: Color::Green,
            warn: Color::Yellow,
            danger: Color::Red,
            gauge_bg: Color::Black,
        }
    }

    pub fn level_color(&self, percent: f64) -> Color {
        if percent < 50.0 {
            self.success
        } else if percent < 80.0 {
            self.warn
        } else {
            self.danger
        }
    }
}
