use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, Paragraph},
    Frame,
};

use crate::app::{App, Screen, Theme};

pub fn render(f: &mut Frame, app: &App) {
    let t = &app.theme;
    
    let full = f.area();
    f.render_widget(Block::default().style(Style::default().bg(t.bg)), full);
    
    let main_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Min(0),
            Constraint::Length(3),
        ])
        .split(f.area());

    let title = Paragraph::new("🧾 License System on Solana - TUI")
        .alignment(Alignment::Center)
        .style(Style::default().fg(t.title).bg(t.bg).add_modifier(Modifier::BOLD))
        .block(Block::default().borders(Borders::ALL).border_style(Style::default().fg(t.border)));
    f.render_widget(title, main_chunks[0]);

    let content_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(30),
            Constraint::Percentage(70),
        ])
        .split(main_chunks[1]);

    render_menu(f, app, content_chunks[0]);
    render_content(f, app, content_chunks[1]);
    render_status_bar(f, app, main_chunks[2]);

    if app.show_help_popup {
        render_help_popup(f, app);
    }

    if let Some(modal) = &app.confirm_modal {
        modal.render(f, &app.theme);
    }

    if let Some(notification) = &app.notification_modal {
        notification.render(f, &app.theme);
    }
}

fn render_menu(f: &mut Frame, app: &App, area: ratatui::layout::Rect) {
    let t = &app.theme;
    
    let menu_title = match app.screen {
        Screen::Settings | Screen::SettingsTheme | Screen::SettingsNetwork => "Settings",
        _ => "Menu",
    };
    
    let items: Vec<ListItem> = app
        .menu_items()
        .iter()
        .enumerate()
        .map(|(i, item)| {
            let is_selected = match app.screen {
                Screen::Settings => i == app.selected,
                Screen::SettingsTheme => i == 1,
                Screen::SettingsNetwork => i == 2,
                _ => i == app.selected,
            };
            
            let style = if is_selected {
                Style::default()
                    .fg(t.accent)
                    .add_modifier(Modifier::BOLD)
            } else {
                Style::default().fg(t.fg).bg(t.bg)
            };
            let prefix = if is_selected { "▸ " } else { "  " };
            ListItem::new(Line::from(Span::styled(format!("{}{}", prefix, item), style)))
        })
        .collect();

    let list = List::new(items).block(
        Block::default()
            .borders(Borders::ALL)
            .border_style(Style::default().fg(t.border))
            .title(menu_title)
            .title_style(Style::default().fg(t.title).add_modifier(Modifier::BOLD)),
    );
    f.render_widget(list, area);
}

fn render_content(f: &mut Frame, app: &App, area: ratatui::layout::Rect) {
    let t = &app.theme;
    
    match app.screen {
        Screen::Main => {
            let help = Paragraph::new(
                "Welcome to License System TUI\n\n\
                Navigate with ↑↓ arrows\n\
                Press Enter to select\n\
                Press ESC to return\n\
                Press q to quit"
            )
            .style(Style::default().fg(t.muted).bg(t.bg))
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .border_style(Style::default().fg(t.border))
                    .title("Help")
                    .title_style(Style::default().fg(t.title).add_modifier(Modifier::BOLD)),
            );
            f.render_widget(help, area);
        }
        Screen::Settings => {
            let help = Paragraph::new(
                "Select an option from the menu\n\n\
                Theme: Change color scheme\n\
                Network: Switch between localnet/devnet/mainnet\n\
                Back: Return to main menu"
            )
            .style(Style::default().fg(t.muted).bg(t.bg))
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .border_style(Style::default().fg(t.border))
                    .title("Info")
                    .title_style(Style::default().fg(t.title).add_modifier(Modifier::BOLD)),
            );
            f.render_widget(help, area);
        }
        Screen::SettingsTheme => {
            let theme_items: Vec<ListItem> = app
                .theme_options()
                .iter()
                .enumerate()
                .map(|(i, name)| {
                    let is_current = name.to_lowercase() == app.theme.name;
                    let style = if i == app.content_selected() {
                        Style::default()
                            .fg(t.accent)
                            .bg(t.highlight)
                            .add_modifier(Modifier::BOLD)
                    } else {
                        Style::default().fg(t.fg).bg(t.bg)
                    };
                    let marker = if is_current { "●" } else { "○" };
                    let prefix = if i == app.content_selected() { "▸ " } else { "  " };
                    ListItem::new(Line::from(Span::styled(
                        format!("{}{} {}", prefix, marker, name),
                        style,
                    )))
                })
                .collect();

            let theme_list = List::new(theme_items).block(
                Block::default()
                    .borders(Borders::ALL)
                    .border_style(Style::default().fg(t.border))
                    .title("Select Theme")
                    .title_style(Style::default().fg(t.title).add_modifier(Modifier::BOLD)),
            );
            f.render_widget(theme_list, area);
        }
        Screen::SettingsNetwork => {
            let network_items: Vec<ListItem> = app
                .network_options()
                .iter()
                .enumerate()
                .map(|(i, name)| {
                    let is_current = name.to_lowercase() == app.network;
                    let style = if i == app.content_selected() {
                        Style::default()
                            .fg(t.accent)
                            .bg(t.highlight)
                            .add_modifier(Modifier::BOLD)
                    } else {
                        Style::default().fg(t.fg).bg(t.bg)
                    };
                    let marker = if is_current { "●" } else { "○" };
                    let prefix = if i == app.content_selected() { "▸ " } else { "  " };
                    ListItem::new(Line::from(Span::styled(
                        format!("{}{} {}", prefix, marker, name),
                        style,
                    )))
                })
                .collect();

            let network_list = List::new(network_items).block(
                Block::default()
                    .borders(Borders::ALL)
                    .border_style(Style::default().fg(t.border))
                    .title("Select Network")
                    .title_style(Style::default().fg(t.title).add_modifier(Modifier::BOLD)),
            );
            f.render_widget(network_list, area);
        }
        Screen::IssueLicense | Screen::ExtendLicense | Screen::ValidateLicense | 
        Screen::RevokeLicense | Screen::ListLicenses => {
            crate::ui::forms::render_form(f, app, area);
        }
    }
}

fn render_status_bar(f: &mut Frame, app: &App, area: ratatui::layout::Rect) {
    let t = &app.theme;
    
    let status_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(50),
            Constraint::Percentage(50),
        ])
        .split(area);

    let status_style = if app.status_message.contains("✅") {
        Style::default().fg(t.success).bg(t.bg)
    } else if app.status_message.contains("❌") {
        Style::default().fg(t.error).bg(t.bg)
    } else {
        Style::default().fg(t.fg).bg(t.bg)
    };

    let status_left = Paragraph::new(app.status_message.as_str())
        .style(status_style)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_style(Style::default().fg(t.border))
                .title("Status")
                .title_style(Style::default().fg(t.title).add_modifier(Modifier::BOLD))
        );
    f.render_widget(status_left, status_chunks[0]);

    let connection_text = format!("Connected to Solana [{}]", app.network);
    let status_right = Paragraph::new(connection_text)
        .style(Style::default().fg(t.success).bg(t.bg))
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_style(Style::default().fg(t.border))
                .title("Network")
                .title_style(Style::default().fg(t.title).add_modifier(Modifier::BOLD))
        );
    f.render_widget(status_right, status_chunks[1]);
}

fn render_help_popup(f: &mut Frame, app: &App) {
    let t = &app.theme;
    
    let help_text = match app.screen {
        Screen::Main => {
            "Main Menu - Keyboard Shortcuts\n\n\
            ↑↓        Navigate menu\n\
            Enter     Select option\n\
            q         Quit\n\
            F1 or ?   Toggle this help\n\n\
            Options:\n\
            - Issue License: Create new license\n\
            - Extend License: Add more days\n\
            - Validate License: Check status\n\
            - Revoke License: Permanently revoke\n\
            - List Licenses: Show license info\n\
            - Settings: Change theme/network"
        }
        Screen::Settings => {
            "Settings - Keyboard Shortcuts\n\n\
            ↑↓        Navigate menu\n\
            Enter     Select option\n\
            ESC       Return to main menu\n\
            F1 or ?   Toggle this help\n\n\
            Options:\n\
            - Theme: Change color scheme\n\
            - Network: Switch network\n\
            - Back: Return to main menu"
        }
        Screen::SettingsTheme => {
            "Theme Selection - Keyboard Shortcuts\n\n\
            ↑↓        Navigate themes\n\
            Enter     Apply theme\n\
            ESC       Return to Settings\n\
            F1 or ?   Toggle this help\n\n\
            Available Themes:\n\
            - Dc Studio: Dark red/burgundy (default)\n\
            - Dark: Dark blue\n\
            - Light: Light theme\n\
            - Dracula: Dracula colors\n\
            - Nord: Nord colors\n\
            - Gruvbox: Gruvbox colors"
        }
        Screen::SettingsNetwork => {
            "Network Selection - Keyboard Shortcuts\n\n\
            ↑↓        Navigate networks\n\
            Enter     Switch network\n\
            ESC       Return to Settings\n\
            F1 or ?   Toggle this help\n\n\
            Available Networks:\n\
            - Localnet: http://127.0.0.1:8899\n\
            - Devnet: https://api.devnet.solana.com\n\
            - Mainnet: https://api.mainnet-beta.solana.com"
        }
        Screen::IssueLicense => {
            "Issue License - Keyboard Shortcuts\n\n\
            Type      Enter data\n\
            Enter     Execute transaction\n\
            ESC       Return to main menu\n\
            F1 or ?   Toggle this help\n\n\
            Format:\n\
            owner_pubkey,product_id,days\n\n\
            Example:\n\
            3whY1ohdAV3uRXSpyzsKtwLg84X9fTZ1pSdCS8Vvqt7c,premium-plan,30"
        }
        Screen::ExtendLicense => {
            "Extend License - Keyboard Shortcuts\n\n\
            Type      Enter data\n\
            Enter     Execute transaction\n\
            ESC       Return to main menu\n\
            F1 or ?   Toggle this help\n\n\
            Format:\n\
            owner_pubkey,additional_days\n\n\
            Example:\n\
            3whY1ohdAV3uRXSpyzsKtwLg84X9fTZ1pSdCS8Vvqt7c,15"
        }
        Screen::ValidateLicense => {
            "Validate License - Keyboard Shortcuts\n\n\
            Type      Enter data\n\
            Enter     Check license\n\
            ESC       Return to main menu\n\
            F1 or ?   Toggle this help\n\n\
            Format:\n\
            owner_pubkey,product_id\n\n\
            Example:\n\
            3whY1ohdAV3uRXSpyzsKtwLg84X9fTZ1pSdCS8Vvqt7c,premium-plan"
        }
        Screen::RevokeLicense => {
            "Revoke License - Keyboard Shortcuts\n\n\
            Type      Enter data\n\
            Enter     Execute transaction\n\
            ESC       Return to main menu\n\
            F1 or ?   Toggle this help\n\n\
            Format:\n\
            owner_pubkey\n\n\
            Example:\n\
            3whY1ohdAV3uRXSpyzsKtwLg84X9fTZ1pSdCS8Vvqt7c\n\n\
            ⚠️  Warning: Revocation is permanent!"
        }
        Screen::ListLicenses => {
            "List Licenses - Keyboard Shortcuts\n\n\
            Type      Enter data\n\
            Enter     Fetch license\n\
            ESC       Return to main menu\n\
            F1 or ?   Toggle this help\n\n\
            Format:\n\
            owner_pubkey\n\n\
            Example:\n\
            3whY1ohdAV3uRXSpyzsKtwLg84X9fTZ1pSdCS8Vvqt7c"
        }
    };

    let area = centered_rect(60, 70, f.area());
    
    f.render_widget(ratatui::widgets::Clear, area);
    
    let popup = Paragraph::new(help_text)
        .style(Style::default().fg(t.fg).bg(t.bg))
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_style(Style::default().fg(t.accent))
                .title("❓ Help - Press F1 or ? to close")
                .title_style(Style::default().fg(t.title).add_modifier(Modifier::BOLD)),
        )
        .wrap(ratatui::widgets::Wrap { trim: true });
    
    f.render_widget(popup, area);
}

fn centered_rect(percent_x: u16, percent_y: u16, r: ratatui::layout::Rect) -> ratatui::layout::Rect {
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage((100 - percent_y) / 2),
            Constraint::Percentage(percent_y),
            Constraint::Percentage((100 - percent_y) / 2),
        ])
        .split(r);

    Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage((100 - percent_x) / 2),
            Constraint::Percentage(percent_x),
            Constraint::Percentage((100 - percent_x) / 2),
        ])
        .split(popup_layout[1])[1]
}
