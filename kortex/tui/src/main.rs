use anyhow::Result;
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::{Backend, CrosstermBackend},
    layout::{Constraint, Direction, Layout},
    widgets::{Block, Borders, Gauge, Paragraph, Wrap},
    Frame, Terminal,
};
use std::io;
use std::time::{Duration, Instant};

struct App {
    mission_step: String,
    verity_score: f32,
    velocity: u64,
    node_count: usize,
}

impl App {
    fn new() -> App {
        App {
            mission_step: "Initializing Hades-Oracle...".to_string(),
            verity_score: 0.0,
            velocity: 0,
            node_count: 0,
        }
    }

    fn on_tick(&mut self) {
        // Update physics/simulated geometric data
        self.velocity = (self.velocity + 1337) % 1000000;
        self.node_count = 1536; // Constant for the unit hypercube manifold
        self.verity_score = (self.verity_score + 0.01).min(1.0);
    }
}

fn main() -> Result<()> {
    // Setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // create app and run it
    let tick_rate = Duration::from_millis(250);
    let mut app = App::new();
    let res = run_app(&mut terminal, &mut app, tick_rate);

    // restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    if let Err(err) = res {
        println!("{:?}", err)
    }

    Ok(())
}

fn run_app<B: Backend>(
    terminal: &mut Terminal<B>,
    app: &mut App,
    tick_rate: Duration,
) -> io::Result<()> {
    let mut last_tick = Instant::now();
    loop {
        terminal.draw(|f| ui(f, app))?;

        let timeout = tick_rate
            .checked_sub(last_tick.elapsed())
            .unwrap_or_else(|| Duration::from_secs(0));
        if crossterm::event::poll(timeout)? {
            if let Event::Key(key) = event::read()? {
                if let KeyCode::Char('q') = key.code {
                    return Ok(());
                }
            }
        }
        if last_tick.elapsed() >= tick_rate {
            app.on_tick();
            last_tick = Instant::now();
        }
    }
}

fn ui(f: &mut Frame, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints(
            [
                Constraint::Length(3),
                Constraint::Min(2),
                Constraint::Length(3),
            ]
            .as_ref(),
        )
        .split(f.size());

    // 1. Mission Status
    let status = Paragraph::new(format!("OBJECTIVE: {}", app.mission_step))
        .block(Block::default().borders(Borders::ALL).title(" MISSION CONTROL [PROJECT HADES] "));
    f.render_widget(status, chunks[0]);

    // 2. Main content (Geometry + Visuals)
    let body_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
        .split(chunks[1]);

    // 2.1 Geometric Manifold Viewport
    let geometry = Paragraph::new(format!(
        "MANIFOLD: PYTHAGOREAN\nDIMENSION: 1536 (Unit Hypercube)\nACTIVE NODES: {}\nGEOMETRIC TRUTH: ENABLED",
        app.node_count
    ))
    .block(Block::default().borders(Borders::ALL).title(" GEOMETRIC VIEWPORT "))
    .wrap(Wrap { trim: true });
    f.render_widget(geometry, body_chunks[0]);

    // 2.2 System Metrics
    let metrics_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(3), Constraint::Min(0)].as_ref())
        .split(body_chunks[1]);

    let verity = Gauge::default()
        .block(Block::default().borders(Borders::ALL).title(" STOIC KATALEPSIS (VERITY) "))
        .gauge_style(ratatui::style::Style::default().fg(ratatui::style::Color::Cyan))
        .percent((app.verity_score * 100.0) as u16);
    f.render_widget(verity, metrics_chunks[0]);

    let velocity = Paragraph::new(format!("VEDIC CALCULATION VELOCITY: {} ops/cycle", app.velocity))
        .block(Block::default().borders(Borders::ALL).title(" COMPUTATIONAL MASTER "))
        .style(ratatui::style::Style::default().fg(ratatui::style::Color::Yellow));
    f.render_widget(velocity, metrics_chunks[1]);

    // 3. Footer
    let footer = Paragraph::new("Press 'q' to disconnect from the Oracle.")
        .block(Block::default().borders(Borders::NONE));
    f.render_widget(footer, chunks[2]);
}
