use anyhow::Result;
use crossterm::{
    event::{self, Event, KeyCode, KeyEvent},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, Paragraph},
    Frame, Terminal,
};
use std::io;

#[derive(Debug, Clone)]
enum Screen {
    Main,
    IssueLicense,
    ExtendLicense,
    ValidateLicense,
    RevokeLicense,
}

struct App {
    screen: Screen,
    selected: usize,
    input: String,
    status_message: String,
}

impl App {
    fn new() -> Self {
        Self {
            screen: Screen::Main,
            selected: 0,
            input: String::new(),
            status_message: String::from("Welcome to License System TUI"),
        }
    }

    fn menu_items(&self) -> Vec<&str> {
        vec![
            "1. Issue License",
            "2. Extend License",
            "3. Validate License",
            "4. Revoke License",
            "5. Exit",
        ]
    }

    fn handle_key(&mut self, key: KeyEvent) -> bool {
        match self.screen {
            Screen::Main => match key.code {
                KeyCode::Char('1') => {
                    self.screen = Screen::IssueLicense;
                    self.status_message = "Issue License - Enter owner pubkey".to_string();
                }
                KeyCode::Char('2') => {
                    self.screen = Screen::ExtendLicense;
                    self.status_message = "Extend License - Enter license PDA".to_string();
                }
                KeyCode::Char('3') => {
                    self.screen = Screen::ValidateLicense;
                    self.status_message = "Validate License - Enter license PDA".to_string();
                }
                KeyCode::Char('4') => {
                    self.screen = Screen::RevokeLicense;
                    self.status_message = "Revoke License - Enter license PDA".to_string();
                }
                KeyCode::Char('5') | KeyCode::Char('q') => return true,
                KeyCode::Down => {
                    self.selected = (self.selected + 1) % self.menu_items().len();
                }
                KeyCode::Up => {
                    if self.selected > 0 {
                        self.selected -= 1;
                    } else {
                        self.selected = self.menu_items().len() - 1;
                    }
                }
                _ => {}
            },
            _ => match key.code {
                KeyCode::Esc => {
                    self.screen = Screen::Main;
                    self.input.clear();
                    self.status_message = "Returned to main menu".to_string();
                }
                KeyCode::Char(c) => {
                    self.input.push(c);
                }
                KeyCode::Backspace => {
                    self.input.pop();
                }
                KeyCode::Enter => {
                    self.execute_action();
                }
                _ => {}
            },
        }
        false
    }

    fn execute_action(&mut self) {
        match self.screen {
            Screen::IssueLicense => {
                self.status_message = format!("Issuing license for: {}", self.input);
            }
            Screen::ExtendLicense => {
                self.status_message = format!("Extending license: {}", self.input);
            }
            Screen::ValidateLicense => {
                self.status_message = format!("Validating license: {}", self.input);
            }
            Screen::RevokeLicense => {
                self.status_message = format!("Revoking license: {}", self.input);
            }
            Screen::Main => {}
        }
        self.input.clear();
    }
}

fn ui(f: &mut Frame, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Min(0),
            Constraint::Length(3),
        ])
        .split(f.area());

    let title = Paragraph::new("License System on Solana - TUI")
        .style(Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD))
        .block(Block::default().borders(Borders::ALL));
    f.render_widget(title, chunks[0]);

    match app.screen {
        Screen::Main => {
            let items: Vec<ListItem> = app
                .menu_items()
                .iter()
                .enumerate()
                .map(|(i, item)| {
                    let style = if i == app.selected {
                        Style::default()
                            .fg(Color::Yellow)
                            .add_modifier(Modifier::BOLD)
                    } else {
                        Style::default()
                    };
                    ListItem::new(Line::from(Span::styled(*item, style)))
                })
                .collect();

            let list = List::new(items).block(
                Block::default()
                    .borders(Borders::ALL)
                    .title("Main Menu - Use ↑↓ or numbers to select"),
            );
            f.render_widget(list, chunks[1]);
        }
        _ => {
            let input_block = Paragraph::new(app.input.as_str())
                .style(Style::default().fg(Color::Yellow))
                .block(
                    Block::default()
                        .borders(Borders::ALL)
                        .title(format!("{:?} - Press ESC to return", app.screen)),
                );
            f.render_widget(input_block, chunks[1]);
        }
    }

    let status = Paragraph::new(app.status_message.as_str())
        .style(Style::default().fg(Color::Green))
        .block(Block::default().borders(Borders::ALL).title("Status"));
    f.render_widget(status, chunks[2]);
}

fn main() -> Result<()> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut app = App::new();

    loop {
        terminal.draw(|f| ui(f, &app))?;

        if event::poll(std::time::Duration::from_millis(100))? {
            if let Event::Key(key) = event::read()? {
                if app.handle_key(key) {
                    break;
                }
            }
        }
    }

    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;

    Ok(())
}
