use ratatui::{
    crossterm::event::{poll, read as event_read, Event, KeyCode, KeyEvent, KeyEventKind},
    widgets::Paragraph,
    DefaultTerminal, Frame,
};
use std::{
    io::Result as IoResult,
    time::{Duration, Instant},
};

pub struct App {
    running: bool,
    ticks: u64,
    key_presses: u64,
    tick_rate: Duration,
}

impl App {
    pub fn new() -> Self {
        Self {
            running: true,
            ticks: 0,
            key_presses: 0,
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

        self.key_presses += 1;

        match key.code {
            KeyCode::Char('q') | KeyCode::Esc => self.running = false,
            _ => {}
        };
    }

    fn on_tick(&mut self) {
        self.ticks += 1;
    }

    fn render(&mut self, frame: &mut Frame) {
        let text = format!(
            "R-System is alive! Ticks: #{}, Key presses: #{}",
            self.ticks, self.key_presses
        );

        frame.render_widget(Paragraph::new(text), frame.area());
    }
}
