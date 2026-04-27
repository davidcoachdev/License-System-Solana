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
use license_sdk::{LicenseClient, License};
use solana_sdk::{pubkey::Pubkey, signature::Keypair};
use std::str::FromStr;

#[derive(Debug, Clone)]
enum Screen {
    Main,
    IssueLicense,
    ExtendLicense,
    ValidateLicense,
    RevokeLicense,
    ListLicenses,
}

struct App {
    screen: Screen,
    selected: usize,
    input: String,
    status_message: String,
    sdk_client: Option<LicenseClient>,
}

impl App {
    fn new() -> Self {
        Self {
            screen: Screen::Main,
            selected: 0,
            input: String::new(),
            status_message: String::from("Welcome to License System TUI - Connecting to Solana..."),
            sdk_client: None,
        }
    }

    fn init_sdk(&mut self, keypair_path: &str) -> Result<()> {
        let keypair = solana_sdk::signature::read_keypair_file(keypair_path)
            .map_err(|e| anyhow::anyhow!("Failed to load keypair: {}", e))?;
        
        let client = LicenseClient::new_localnet(keypair)?;
        self.sdk_client = Some(client);
        self.status_message = "Connected to Solana localnet".to_string();
        Ok(())
    }

    fn menu_items(&self) -> Vec<&str> {
        vec![
            "1. Issue License",
            "2. Extend License",
            "3. Validate License",
            "4. Revoke License",
            "5. List Licenses",
            "6. Exit",
        ]
    }

    fn handle_key(&mut self, key: KeyEvent) -> bool {
        match self.screen {
            Screen::Main => match key.code {
                KeyCode::Char('1') => {
                    self.screen = Screen::IssueLicense;
                    self.status_message = "Issue License - Enter: owner_pubkey,product_id,days".to_string();
                }
                KeyCode::Char('2') => {
                    self.screen = Screen::ExtendLicense;
                    self.status_message = "Extend License - Enter: owner_pubkey,additional_days".to_string();
                }
                KeyCode::Char('3') => {
                    self.screen = Screen::ValidateLicense;
                    self.status_message = "Validate License - Enter: owner_pubkey,product_id".to_string();
                }
                KeyCode::Char('4') => {
                    self.screen = Screen::RevokeLicense;
                    self.status_message = "Revoke License - Enter: owner_pubkey".to_string();
                }
                KeyCode::Char('5') => {
                    self.screen = Screen::ListLicenses;
                    self.status_message = "List Licenses - Enter: owner_pubkey".to_string();
                }
                KeyCode::Char('6') | KeyCode::Char('q') => return true,
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
        if self.sdk_client.is_none() {
            self.status_message = "Error: SDK not initialized".to_string();
            return;
        }

        let client = self.sdk_client.as_ref().unwrap();

        match self.screen {
            Screen::IssueLicense => {
                let parts: Vec<&str> = self.input.split(',').collect();
                if parts.len() != 3 {
                    self.status_message = "Format: owner_pubkey,product_id,days".to_string();
                    return;
                }
                
                let owner = match Pubkey::from_str(parts[0].trim()) {
                    Ok(pk) => pk,
                    Err(_) => {
                        self.status_message = "Invalid owner pubkey".to_string();
                        return;
                    }
                };
                
                let product_id = parts[1].trim().to_string();
                let days: i64 = match parts[2].trim().parse() {
                    Ok(d) => d,
                    Err(_) => {
                        self.status_message = "Invalid days number".to_string();
                        return;
                    }
                };

                let (license_pda, bump) = client.derive_license_pda(&owner);
                self.status_message = format!(
                    "✅ Ready to issue license!\nPDA: {}\nBump: {}\nOwner: {}\nProduct: {}\nDays: {}\nPayer: {}\n\n[Demo mode - use TypeScript tests for real transactions]",
                    license_pda, bump, owner, product_id, days, client.payer_pubkey()
                );
            }
            Screen::ExtendLicense => {
                let parts: Vec<&str> = self.input.split(',').collect();
                if parts.len() != 2 {
                    self.status_message = "Format: owner_pubkey,additional_days".to_string();
                    return;
                }
                
                let owner = match Pubkey::from_str(parts[0].trim()) {
                    Ok(pk) => pk,
                    Err(_) => {
                        self.status_message = "Invalid owner pubkey".to_string();
                        return;
                    }
                };
                
                let days: i64 = match parts[1].trim().parse() {
                    Ok(d) => d,
                    Err(_) => {
                        self.status_message = "Invalid days number".to_string();
                        return;
                    }
                };

                let (license_pda, bump) = client.derive_license_pda(&owner);
                self.status_message = format!(
                    "✅ Ready to extend license!\nPDA: {}\nBump: {}\nAdditional days: {}\n\n[Demo mode - use TypeScript tests for real transactions]",
                    license_pda, bump, days
                );
            }
            Screen::ValidateLicense => {
                let parts: Vec<&str> = self.input.split(',').collect();
                if parts.len() != 2 {
                    self.status_message = "Format: owner_pubkey,product_id".to_string();
                    return;
                }
                
                let owner = match Pubkey::from_str(parts[0].trim()) {
                    Ok(pk) => pk,
                    Err(_) => {
                        self.status_message = "Invalid owner pubkey".to_string();
                        return;
                    }
                };
                
                let product_id = parts[1].trim();
                let (license_pda, bump) = client.derive_license_pda(&owner);
                self.status_message = format!(
                    "✅ Ready to validate license!\nPDA: {}\nBump: {}\nProduct: {}\n\n[Demo mode - use TypeScript tests for real validation]",
                    license_pda, bump, product_id
                );
            }
            Screen::RevokeLicense => {
                let owner = match Pubkey::from_str(self.input.trim()) {
                    Ok(pk) => pk,
                    Err(_) => {
                        self.status_message = "Invalid owner pubkey".to_string();
                        return;
                    }
                };

                let (license_pda, bump) = client.derive_license_pda(&owner);
                self.status_message = format!(
                    "✅ Ready to revoke license!\nPDA: {}\nBump: {}\n\n[Demo mode - use TypeScript tests for real transactions]",
                    license_pda, bump
                );
            }
            Screen::ListLicenses => {
                let owner = match Pubkey::from_str(self.input.trim()) {
                    Ok(pk) => pk,
                    Err(_) => {
                        self.status_message = "Invalid owner pubkey".to_string();
                        return;
                    }
                };
                
                let (license_pda, bump) = client.derive_license_pda(&owner);
                self.status_message = format!(
                    "📋 License Info:\nPDA: {}\nBump: {}\nOwner: {}\nPayer: {}\nProgram ID: {}\n\n[Demo mode - showing PDA derivation only]",
                    license_pda, bump, owner, client.payer_pubkey(), client.program_id()
                );
            }
            Screen::Main => {
                return;
            }
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
    let mut app = App::new();
    
    let keypair_path = std::env::var("ANCHOR_WALLET")
        .unwrap_or_else(|_| format!("{}/.config/solana/id.json", std::env::var("HOME").unwrap()));
    
    if let Err(e) = app.init_sdk(&keypair_path) {
        eprintln!("Failed to initialize SDK: {}", e);
        eprintln!("Make sure ANCHOR_WALLET is set or ~/.config/solana/id.json exists");
        return Err(e);
    }

    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

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
