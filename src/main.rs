use std::io::stdout;
use ratatui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout},
    widgets::{Block, Borders, Paragraph},
    Terminal,
};

fn main() {
    let stdout = stdout();
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend).unwrap();

    loop {
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

                let stopwatch_text = Paragraph::new("I'm a stopwatch").block(stopwatch_block);
                let utc_text = Paragraph::new("Hi I'm in London").block(utc_time_block);

                f.render_widget(stopwatch_text, stopwatch_area);
                f.render_widget(utc_text, utc_time_area);
            })
            .unwrap();
        std::thread::sleep(std::time::Duration::from_millis(20));
        terminal.clear().unwrap();
    }
}
