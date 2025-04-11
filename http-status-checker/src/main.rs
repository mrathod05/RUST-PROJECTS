use crossterm::{
    event::{self, Event, KeyCode, KeyEvent, KeyModifiers},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use reqwest::StatusCode;
use std::{collections::HashMap, io, sync::Arc, time::Duration};
use tokio::{
    sync::{mpsc, RwLock},
    task::JoinSet,
};
use tui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Span, Spans},
    widgets::{Block, Borders, List, ListItem, Paragraph},
    Terminal,
};

use url::Url;

#[derive(Debug, Clone)]
enum StatusResult {
    Pending,
    Success(StatusCode),
    Error(String),
}

#[derive(Clone)]
struct AppState {
    urls: HashMap<String, StatusResult>,
    input_buffer: String,
    should_quit: bool,
}

impl AppState {
    fn new() -> Self {
        Self {
            urls: HashMap::new(),
            input_buffer: String::new(),
            should_quit: false,
        }
    }

    fn add_url(&mut self, url: String) -> bool {
        if url.is_empty() || self.urls.contains_key(&url) {
            return false;
        }

        if Url::parse(&url).is_err() {
            self.urls
                .insert(url, StatusResult::Error("Invalid URL".to_string()));

            return false;
        }

        self.urls.insert(url, StatusResult::Pending);
        return true;
    }

    fn update_status(&mut self, url: &String, status: StatusResult) {
        if self.urls.contains_key(url) {
            self.urls.insert(url.to_string(), status);
        }
    }
}

async fn fetch_status_code(url: String) -> (String, StatusResult) {
    let client = reqwest::Client::new();

    let result = match client
        .get(&url)
        .timeout(Duration::from_secs(5))
        .send()
        .await
    {
        Ok(res) => StatusResult::Success(res.status()),
        Err(err) => {
            if err.is_timeout() {
                StatusResult::Error("Request timed out".to_string())
            } else {
                StatusResult::Error(err.to_string())
            }
        }
    };
    (url, result)
}

async fn run_ui() -> Result<(), Box<dyn std::error::Error>> {
    // Terminal setup
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;

    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    terminal.clear()?;

    // Application state
    let app_state = Arc::new(RwLock::new(AppState::new()));

    let (tx, mut rx) = mpsc::channel::<String>(100);

    let worker_state = Arc::clone(&app_state);

    tokio::spawn(async move {
        let mut tasks: JoinSet<(String, StatusResult)> = JoinSet::new();

        loop {
            tokio::select! {
                Some(url) =rx.recv()=>{
                    let trimmed_url =  url.trim().to_string();


                        let mut state = worker_state.write().await;
                        state.update_status(&trimmed_url, StatusResult::Pending);



                    tasks.spawn(async move {fetch_status_code(url).await});
                }

                Some (Ok((url,status))) = tasks.join_next()=>{
                    let mut state = worker_state.write().await;
                    state.update_status(&url,status);
                  },

                  else => break,
            }
        }
    });

    // Main event loop
    loop {
        {
            let state = app_state.read().await;
            if state.should_quit {
                break;
            }
        }

        // Draw UI
        let state_snapshot = {
            let state = app_state.read().await;
            state.clone()
        };

        terminal.draw(|f| {
            let size = f.size();
            let layout = Layout::default()
                .direction(Direction::Vertical)
                .constraints([Constraint::Percentage(80), Constraint::Percentage(20)].as_ref())
                .split(size);

            // Status list
            let status_items: Vec<ListItem> = state_snapshot
                .urls
                .iter()
                .map(|(url, status)| {
                    let (display_text, style) = match status {
                        StatusResult::Success(status_code) => {
                            let color = match status_code.as_u16() {
                                200..=299 => Color::Green,
                                300..=399 => Color::Yellow,
                                400..=499 => Color::Red,
                                _ => Color::Magenta,
                            };
                            (
                                format!("{}: {}", url, status_code),
                                Style::default().fg(color),
                            )
                        }
                        StatusResult::Error(err) => (
                            format!("{}: Error - {}", url, err),
                            Style::default().fg(Color::Red),
                        ),
                        StatusResult::Pending => (
                            format!("{}: Pending...", url),
                            Style::default()
                                .fg(Color::Blue)
                                .add_modifier(Modifier::ITALIC),
                        ),
                    };

                    ListItem::new(Spans::from(Span::styled(display_text, style)))
                })
                .collect();

            let status_list = List::new(status_items)
                .block(Block::default().title("Status Codes").borders(Borders::ALL));

            f.render_widget(status_list, layout[0]);

            // Input area
            let input_paragraph = Paragraph::new(state_snapshot.input_buffer.as_str()).block(
                Block::default()
                    .title("Enter URL (Enter to add, Esc to quit)")
                    .borders(Borders::ALL),
            );

            f.render_widget(input_paragraph, layout[1]);

            // Set cursor position
            let max_cursor_x = layout[1].x + layout[1].width.saturating_sub(2);
            let cursor_x =
                (layout[1].x + state_snapshot.input_buffer.len() as u16 + 1).min(max_cursor_x);
            let cursor_y = layout[1].y + 1;
            f.set_cursor(cursor_x, cursor_y);
        })?;

        // Handle input events with timeout
        if crossterm::event::poll(Duration::from_millis(100))? {
            if let Event::Key(KeyEvent {
                code, modifiers, ..
            }) = event::read()?
            {
                let mut state = app_state.write().await;

                match code {
                    KeyCode::Enter => {
                        let url = state.input_buffer.trim().to_string();
                        if !url.is_empty() {
                            if state.add_url(url.clone()) {
                                let _ = tx.try_send(url.clone());
                            }
                            state.input_buffer.clear();
                        }
                    }
                    KeyCode::Char(c) => {
                        if modifiers.contains(KeyModifiers::CONTROL) && c == 'c' {
                            state.should_quit = true;
                        } else {
                            state.input_buffer.push(c);
                        }
                    }
                    KeyCode::Backspace => {
                        state.input_buffer.pop();
                    }
                    KeyCode::Esc => {
                        state.should_quit = true;
                    }
                    _ => {}
                }
            }
        }
    }

    // Cleanup
    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;

    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    run_ui().await?;

    Ok(())
}
