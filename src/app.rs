// Imports -- begin
use crate::{
    math::{percent, push_history, to_gbs, to_points},
    ui::UI,
};
use ratatui::{
    crossterm::event::{poll, read as event_read, Event, KeyCode, KeyEvent, KeyEventKind},
    layout::{Constraint, Layout, Rect},
    style::{Style, Stylize},
    symbols::Marker,
    text::{Line, Span},
    widgets::{
        Axis, Block, BorderType, Chart, Dataset, Gauge, GraphType, LineGauge, Paragraph, Sparkline,
    },
    DefaultTerminal, Frame,
};
use std::{
    collections::VecDeque,
    io::Result as IoResult,
    time::{Duration, Instant},
};
use sysinfo::{Disks, System, MINIMUM_CPU_UPDATE_INTERVAL};
// Imports -- end

pub struct App {
    running: bool,
    system: System,
    host_name: String,
    os_name: String,
    disks: Disks,
    cpu_history: VecDeque<f64>,
    ram_history: VecDeque<f64>,
    // constants
    tick_rate: Duration,
    history_len: usize,
    ui: UI,
}

impl App {
    pub fn new() -> Self {
        let mut system = System::new_all();
        std::thread::sleep(MINIMUM_CPU_UPDATE_INTERVAL);
        system.refresh_cpu_usage();

        let host_name = System::host_name().unwrap_or_else(|| String::from("Unknown host"));
        let os_name = System::long_os_version().unwrap_or_else(|| String::from("Unknown OS"));
        let tick_rate = Duration::from_millis(500);

        let history_len = 120;
        let disks = Disks::new_with_refreshed_list();
        let cpu_history = VecDeque::with_capacity(history_len);
        let ram_history = VecDeque::with_capacity(history_len);

        let mut app = Self {
            running: true,
            system,
            host_name,
            os_name,
            disks,
            cpu_history,
            tick_rate,
            history_len,
            ui: UI::default(),
            ram_history,
        };

        app.on_tick();
        app
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

    fn on_tick(&mut self) {
        self.system.refresh_cpu_usage();
        self.system.refresh_memory();
        self.disks.refresh(true);

        let ram = percent(self.system.used_memory(), self.system.total_memory());

        push_history(&mut self.ram_history, ram, self.history_len);
        push_history(
            &mut self.cpu_history,
            self.system.global_cpu_usage() as f64,
            self.history_len,
        );
    }

    fn render(&self, frame: &mut Frame) {
        let core_rows = self.system.cpus().len().div_ceil(2) as u16;
        let top_height = (core_rows + 3).max(5) + 2;

        let [header, top, history, processes, footer] = Layout::vertical([
            Constraint::Length(3),
            Constraint::Length(top_height),
            Constraint::Length(10),
            Constraint::Fill(1),
            Constraint::Length(1),
        ])
        .areas(frame.area());

        let [cpu, memory] =
            Layout::horizontal([Constraint::Percentage(50), Constraint::Percentage(50)]).areas(top);

        self.render_header(frame, header);
        self.render_cpu(frame, cpu);
        self.render_memory(frame, memory);
        self.render_history(frame, history);
        frame.render_widget(self.panel("Processes"), processes);
        self.render_footer(frame, footer);
    }

    fn render_header(&self, frame: &mut Frame, area: Rect) {
        let separator = Span::styled(" | ", self.ui.muted);
        let uptime = System::uptime(); // Duration in seconds

        let hours = uptime / 3600;
        let minutes = uptime % 3600 / 60;

        let line = Line::from(vec![
            "R-System Monitor".bold().fg(self.ui.accent),
            separator.clone(),
            self.host_name.clone().bold(),
            separator.clone(),
            Span::raw(format!("Uptime: {} hours {} minutes", hours, minutes)),
        ]);

        let block = Block::bordered()
            .border_type(BorderType::Rounded)
            .border_style(self.ui.muted);

        frame.render_widget(Paragraph::new(line).block(block), area);
    }

    fn render_footer(&self, frame: &mut Frame, area: Rect) {
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

    fn render_cpu(&self, frame: &mut Frame, area: Rect) {
        let average = self.system.global_cpu_usage();
        let block = self.panel(&format!("CPU {:.0}%", average));

        let inner = block.inner(area);
        frame.render_widget(block, area);

        let core_rows = self.system.cpus().len().div_ceil(2) as u16;

        let [cores_area, spark_area] =
            Layout::vertical([Constraint::Length(core_rows), Constraint::Fill(1)])
                .spacing(2)
                .areas(inner);

        let [left, right] = Layout::horizontal([Constraint::Fill(1), Constraint::Fill(1)])
            .spacing(2)
            .areas(cores_area);

        for (idx, cpu) in self.system.cpus().iter().enumerate() {
            let usage = cpu.cpu_usage() as f64;
            let col = if idx & 1 == 0 { left } else { right };
            let row = Rect {
                y: col.y + (idx >> 1) as u16,
                height: 1,
                ..col
            };

            let gauge = LineGauge::default()
                .ratio((usage / 100.0).clamp(0.0, 1.0))
                .label(format!("{:>2} {:>3.0}%", idx + 1, usage))
                .filled_symbol("█")
                .unfilled_symbol("░")
                .filled_style(self.ui.level_color(usage))
                .unfilled_style(self.ui.muted);

            frame.render_widget(gauge, row);
        }

        let width = spark_area.width as usize;
        let skip = self.cpu_history.len().saturating_sub(width);

        let data: Vec<u64> = self
            .cpu_history
            .iter()
            .skip(skip)
            .map(|v| *v as u64)
            .collect();

        let sparkline = Sparkline::default()
            .data(&data)
            .max(100)
            .style(self.ui.accent);

        frame.render_widget(sparkline, spark_area);
    }

    fn render_memory(&self, frame: &mut Frame, area: Rect) {
        let block = self.panel("Memory");
        let inner = block.inner(area);
        frame.render_widget(block, area);

        let [ram_area, swap_area, disk_area] = Layout::vertical([Constraint::Length(1); 3])
            .spacing(1)
            .areas(inner);

        let sys = &self.system;

        frame.render_widget(
            self.used_gauge("RAM", sys.used_memory(), sys.total_memory()),
            ram_area,
        );
        frame.render_widget(
            self.used_gauge("Swap", sys.used_swap(), sys.total_swap()),
            swap_area,
        );

        if let Some(disk) = self.disks.list().first() {
            let total = disk.total_space();
            let used = total - disk.available_space();
            let label = format!("(Disk {})", disk.mount_point().display());

            frame.render_widget(self.used_gauge(&label, used, total), disk_area);
        }
    }

    fn render_history(&self, frame: &mut Frame, area: Rect) {
        let cpu_points = to_points(&self.cpu_history, self.history_len);
        let ram_points = to_points(&self.ram_history, self.history_len);

        let datasets = vec![
            Dataset::default()
                .name("CPU")
                .marker(Marker::Braille)
                .graph_type(GraphType::Line)
                .style(self.ui.accent)
                .data(&cpu_points),
            Dataset::default()
                .name("RAM")
                .marker(Marker::Braille)
                .graph_type(GraphType::Line)
                .style(self.ui.ram_color)
                .data(&ram_points),
        ];

        let x_axis = Axis::default()
            .bounds([0.0, (self.history_len - 1) as f64])
            .labels(["60s ago", "30s", "now"])
            .style(self.ui.muted);
        let y_axis = Axis::default()
            .bounds([0.0, 100.0])
            .labels(["0%", "50%", "100%"])
            .style(self.ui.muted);

        let legend = Line::from(vec![
            (" ● CPU ".fg(self.ui.accent)),
            (" ● RAM ".fg(self.ui.ram_color)),
        ])
        .right_aligned();

        let block = self.panel("History · 60s").title_top(legend);

        let chart = Chart::new(datasets)
            .block(block)
            .x_axis(x_axis)
            .y_axis(y_axis)
            .legend_position(None);

        frame.render_widget(chart, area);
    }

    fn panel(&self, title: &str) -> Block<'static> {
        Block::bordered()
            .border_type(BorderType::Rounded)
            .border_style(self.ui.muted)
            .title(Line::from(format!(" {} ", title)).fg(self.ui.accent).bold())
    }

    fn used_gauge(&self, name: &str, used: u64, total: u64) -> Gauge<'static> {
        let pct = percent(used, total);

        Gauge::default()
            .ratio(pct / 100.0)
            .label(format!(
                "{} {:.1} / {:.1} GB ({:.0}%)",
                name,
                to_gbs(used),
                to_gbs(total),
                pct,
            ))
            .gauge_style(
                Style::new()
                    .fg(self.ui.level_color(pct))
                    .bg(self.ui.gauge_bg),
            )
            .use_unicode(true)
    }
}
