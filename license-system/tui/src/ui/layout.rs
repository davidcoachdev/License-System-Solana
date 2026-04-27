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
}

fn render_menu(f: &mut Frame, app: &App, area: ratatui::layout::Rect) {
    let t = &app.theme;
    
    let menu_title = match app.screen {
        Screen::Settings => "Settings",
        Screen::SettingsTheme => "Theme",
        Screen::SettingsNetwork => "Network",
        _ => "Menu",
    };
    
    let items: Vec<ListItem> = app
        .menu_items()
        .iter()
        .enumerate()
        .map(|(i, item)| {
            let style = if i == app.selected {
                Style::default()
                    .fg(t.accent)
                    .bg(t.highlight)
                    .add_modifier(Modifier::BOLD)
            } else {
                Style::default().fg(t.fg).bg(t.bg)
            };
            let prefix = if i == app.selected { "▸ " } else { "  " };
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
        Screen::Settings | Screen::SettingsTheme | Screen::SettingsNetwork => {
            let help_text = match app.screen {
                Screen::Settings => "Select an option from the menu\n\n\
                    Theme: Change color scheme\n\
                    Network: Switch between localnet/devnet/mainnet\n\
                    Back: Return to main menu",
                Screen::SettingsTheme => "Select a theme from the menu\n\n\
                    Dark: Dark blue theme (default)\n\
                    Light: Light theme\n\
                    Dracula: Dracula theme\n\
                    Nord: Nord theme\n\
                    Gruvbox: Gruvbox theme",
                Screen::SettingsNetwork => "Select a network from the menu\n\n\
                    Localnet: http://127.0.0.1:8899\n\
                    Devnet: https://api.devnet.solana.com\n\
                    Mainnet: https://api.mainnet-beta.solana.com",
                _ => "",
            };
            
            let help = Paragraph::new(help_text)
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
        _ => {
            let input_block = Paragraph::new(app.input.as_str())
                .style(Style::default().fg(t.accent).bg(t.bg))
                .block(
                    Block::default()
                        .borders(Borders::ALL)
                        .border_style(Style::default().fg(t.border))
                        .title(format!("{:?} - Press ESC to return", app.screen))
                        .title_style(Style::default().fg(t.title).add_modifier(Modifier::BOLD)),
                );
            f.render_widget(input_block, area);
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
