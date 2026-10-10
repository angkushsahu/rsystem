// Imports -- begin
use ratatui::{
    crossterm::event::{poll, read as event_read, Event, KeyCode, KeyEvent, KeyEventKind},
    layout::{Constraint, Layout},
    widgets::{Block, Paragraph},
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
}

impl App {
    pub fn new() -> Self {
        Self {
            running: true,
            tick_rate: Duration::from_millis(500),
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

        frame.render_widget(Block::bordered().title("R-System Monitor"), header);
        frame.render_widget(Block::bordered().title("CPU"), cpu);
        frame.render_widget(Block::bordered().title("Memory"), memory);
        frame.render_widget(Block::bordered().title("History"), history);
        frame.render_widget(Block::bordered().title("Processes"), processes);
        frame.render_widget(Paragraph::new("q or Esc - quit"), footer);
    }
}
