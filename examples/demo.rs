use std::time::Duration;

use color_eyre::Result;
use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use itertools::Itertools;
use rand::{thread_rng, Rng};
use ratatui::{DefaultTerminal, Frame};
use tui_equalizer::{Band, Equalizer};

fn main() -> Result<()> {
    color_eyre::install()?;
    let terminal = ratatui::init();
    let result = run(terminal);
    ratatui::restore();
    result
}

fn run(mut terminal: DefaultTerminal) -> Result<()> {
    let width = terminal.size()?.width;
    let mut current_bands = random_bands(width);
    let mut next_bands = random_bands(width);
    let mut last_time = std::time::Instant::now();
    // Update the bands every 500ms
    const UPDATE_INTERVAL: Duration = Duration::from_millis(500);
    loop {
        let percent = last_time.elapsed().as_secs_f64() / UPDATE_INTERVAL.as_secs_f64();
        let interpolated = interpolate(&current_bands, &next_bands, percent);
        if percent >= 1.0 {
            last_time = std::time::Instant::now();
            current_bands = interpolated.clone();
            next_bands = random_bands(width);
        }
        terminal.draw(|frame| draw(frame, &interpolated))?;
        if handle_input()? == Command::Quit {
            break Ok(());
        }
    }
}

fn interpolate(current: &Vec<Band>, next: &Vec<Band>, percent: f64) -> Vec<Band> {
    let interpolated = current
        .iter()
        .zip(next.iter())
        .map(|(current, next)| Band {
            value: current.value + (next.value - current.value) * percent.clamp(0.0, 1.0),
        })
        .collect_vec();
    interpolated
}

fn random_bands(count: u16) -> Vec<Band> {
    (0..count / 2)
        .map(|_| Band::from(thread_rng().gen_range(0.1..1.0)))
        .collect_vec()
}

fn draw(frame: &mut Frame, bands: &[Band]) {
    let size = frame.area();
    let equalizer = Equalizer {
        bands: bands.to_vec(),
    };
    frame.render_widget(equalizer, size);
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum Command {
    Noop,
    Quit,
}

fn handle_input() -> Result<Command> {
    if !event::poll(Duration::from_secs_f64(1.0 / 60.0))? {
        return Ok(Command::Noop);
    }
    match event::read()? {
        Event::Key(key) if key.kind == KeyEventKind::Press => match key.code {
            KeyCode::Char('q') => Ok(Command::Quit),
            _ => Ok(Command::Noop),
        },
        _ => Ok(Command::Noop),
    }
}
