use ratatui::style::Color;

pub struct UI {
    pub muted: Color,
    pub accent: Color,
    pub footer_bg: Color,
    pub success: Color,
    pub warn: Color,
    pub danger: Color,
    pub gauge_bg: Color,
    pub ram_color: Color,
    pub process_row_highlight: Color,
    pub process_highlight_text: Color,
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
            ram_color: Color::Magenta,
            process_row_highlight: Color::Blue,
            process_highlight_text: Color::White,
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
