// Imports -- begin
use crate::ui::UI;
use ratatui::{
    crossterm::event::{poll, read as event_read, Event, KeyCode, KeyEvent, KeyEventKind},
    layout::{Constraint, Layout, Rect},
    style::{Style, Stylize},
    text::{Line, Span},
    widgets::{Block, BorderType, Paragraph},
    DefaultTerminal, Frame,
};
use std::{
    io::Result as IoResult,
    time::{Duration, Instant},
};
// Imports -- end

pub struct App {
    running: bool,
    tick_rate: Duration,
    ui: UI,
}

impl App {
    pub fn new() -> Self {
        Self {
            running: true,
            tick_rate: Duration::from_millis(500),
            ui: UI::default(),
        }
    }

    pub fn run(&mut self, term: &mut DefaultTerminal) -> IoResult<()> {
        let mut last_tick = Instant::now();

        while self.running {
            term.draw(|frame| self.render(frame))?;

            let timeout = self.tick_rate.saturating_sub(last_tick.elapsed());

            #[allow(clippy::collapsible_if)]
            if poll(timeout)? {
                if let Event::Key(key) = event_read()? {
                    self.handle_key(key);
                }
            }

            if last_tick.elapsed() >= self.tick_rate {
                self.on_tick();
                last_tick = Instant::now();
            }
        }

        Ok(())
    }

    fn handle_key(&mut self, key: KeyEvent) {
        if key.kind != KeyEventKind::Press {
            return;
        }

        match key.code {
            KeyCode::Char('q') | KeyCode::Esc => self.running = false,
            _ => {}
        };
    }

    fn on_tick(&mut self) {}

    fn render(&mut self, frame: &mut Frame) {
        let [header, top, history, processes, footer] = Layout::vertical([
            Constraint::Length(3),
            Constraint::Length(10),
            Constraint::Length(10),
            Constraint::Fill(1),
            Constraint::Length(1),
        ])
        .areas(frame.area());

        let [cpu, memory] =
            Layout::horizontal([Constraint::Percentage(50), Constraint::Percentage(50)]).areas(top);

        self.render_header(frame, header);
        frame.render_widget(self.panel("CPU"), cpu);
        frame.render_widget(self.panel("Memory"), memory);
        frame.render_widget(self.panel("History"), history);
        frame.render_widget(self.panel("Processes"), processes);
        // frame.render_widget(Paragraph::new("Press 'q' or 'Esc' to quit"), footer);
        self.render_footer(frame, footer);
    }

    fn render_header(&mut self, frame: &mut Frame, area: Rect) {
        let separator = Span::styled(" | ", self.ui.muted);

        let line = Line::from(vec![
            "R-System Monitor".bold().fg(self.ui.accent),
            separator,
            Span::raw("Live System Monitor"),
        ]);

        let block = Block::bordered()
            .border_type(BorderType::Rounded)
            .border_style(self.ui.muted);

        frame.render_widget(Paragraph::new(line).block(block), area);
    }

    fn render_footer(&mut self, frame: &mut Frame, area: Rect) {
        let key = |k: &'static str, desc: &'static str| {
            [
                Span::styled(
                    format!(" {} ", k),
                    Style::new().fg(self.ui.footer_bg).bg(self.ui.accent),
                ),
                Span::styled(format!(" {}\t", desc), self.ui.muted),
            ]
        };

        let spans: Vec<Span> = [key("q or Esc", "Quit")].into_iter().flatten().collect();

        frame.render_widget(Line::from(spans), area);
    }

    fn panel(&mut self, title: &str) -> Block<'static> {
        Block::bordered()
            .border_type(BorderType::Rounded)
            .border_style(self.ui.muted)
            .title(Line::from(format!(" {} ", title)).fg(self.ui.accent).bold())
    }
}
