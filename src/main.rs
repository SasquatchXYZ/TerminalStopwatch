use std::{
    io::stdout,
    time::Instant,
};

use crossterm::event::{poll, read, Event, KeyCode, KeyEventKind};
use ratatui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout},
    widgets::{Block, Borders, Paragraph},
    Terminal,
};

struct Stopwatch {
    now: Instant,
    state: StopwatchState,
    display: String,
}

enum StopwatchState {
    NotStarted,
    Running,
    Done,
}

impl Stopwatch {
    fn new() -> Self {
        Self {
            now: Instant::now(),
            state: StopwatchState::NotStarted,
            display: String::from("0:00:00"),
        }
    }
    fn get_time(&self) -> String {
        use StopwatchState::*;
        match self.state {
            NotStarted => String::from("0:00:00"),
            Running => {
                let mut elapsed = self.now.elapsed().as_millis();
                let minutes = elapsed / 60000;
                elapsed -= minutes * 60000;
                let seconds = elapsed / 1000;
                elapsed -= seconds * 1000;
                let split_seconds = elapsed / 10;
                format!("{minutes}:{seconds}:{split_seconds}")
            }
            Done => self.display.clone(),
        }
    }
    fn next_state(&mut self) {
        use StopwatchState::*;
        match self.state {
            NotStarted => {
                self.now = Instant::now();
                self.state = Running;
            }
            Running => {
                self.display = self.get_time();
                self.state = Done;
            }
            Done => self.state = NotStarted,
        }
    }
}

fn main() {
    let stdout = stdout();
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend).unwrap();
    let mut stopwatch = Stopwatch::new();

    loop {
        if poll(std::time::Duration::from_millis(0)).unwrap() {
            if let Event::Key(key_event) = read().unwrap() {
                if let (KeyCode::Enter, KeyEventKind::Press) = (key_event.code, key_event.kind) {
                    stopwatch.next_state();
                }
            }
        }

        terminal
            .draw(|f| {
                let layout = Layout::default()
                    .direction(Direction::Horizontal)
                    .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
                    .split(f.area());
                let stopwatch_area = layout[0];
                let utc_time_area = layout[1];

                let stopwatch_block = Block::default().title("Stopwatch").borders(Borders::ALL);
                let utc_time_block = Block::default().title("UTC Time").borders(Borders::ALL);

                let stopwatch_text = Paragraph::new(stopwatch.get_time())
                    .block(stopwatch_block);
                let utc_text = Paragraph::new(
                    chrono::offset::Utc::now().format("%Y/%m/%d %H:%M:%S").to_string()
                ).block(utc_time_block);

                f.render_widget(stopwatch_text, stopwatch_area);
                f.render_widget(utc_text, utc_time_area);
            })
            .unwrap();
        std::thread::sleep(std::time::Duration::from_millis(20));
        terminal.clear().unwrap();
    }
}
