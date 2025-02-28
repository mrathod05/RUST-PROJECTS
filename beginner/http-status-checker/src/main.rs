use clap::{self, Parser, ValueHint};
use crossterm::{
    event::{self, EnableMouseCapture, KeyCode, KeyEvent},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use reqwest::StatusCode;
use std::io;
use tui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout},
    widgets::{Block, Borders, Paragraph},
    Terminal,
};

#[derive(Parser, Debug)]
#[clap(author,version,about,long_about=None)]
struct Args {
    /// URL to check status code
    #[clap(value_hint=ValueHint::Url)]
    urls: Vec<String>,
}

async fn fetch_status_code(url: &str) -> Result<StatusCode, reqwest::Error> {
    return match reqwest::get(url).await {
        Ok(response) => {
            let status = response.status();
            Ok(status)
        }
        Err(err) => {
            eprintln!("Error fetching {}:{}", url, err);
            Err(err)
        }
    };
}

async fn run_ui() -> std::result::Result<(), Box<dyn std::error::Error>> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;

    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let input_buffer = String::new();
    let status_code = fetch_status_code("https://www.google.com/").await?;

    loop {
        terminal.draw(|f| {
            let size = f.size();

            let layout = Layout::default()
                .direction(Direction::Vertical)
                .constraints([Constraint::Percentage(80), Constraint::Percentage(20)].as_ref())
                .split(size);

            let status_paragraph = Paragraph::new(status_code.to_string())
                .block(Block::default().title("Status Codes").borders(Borders::ALL));
            f.render_widget(status_paragraph, layout[0]);

            let input_paragraph = Paragraph::new(input_buffer.as_str())
                .block(Block::default().title("Enter URL").borders(Borders::ALL));
            f.render_widget(input_paragraph, layout[1]);
        })?;

        if let event::Event::Key(KeyEvent { code, .. }) = event::read()? {
            if code == KeyCode::Esc {
                break;
            }
        }
    }

    // Restore terminal settings
    disable_raw_mode().expect("Error");
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;

    Ok(())
}

#[tokio::main]
async fn main() {
    let args = Args::parse();
    let _ = run_ui().await;
}
