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
    
    let items: Vec<ListItem> = app
        .menu_items()
        .iter()
        .enumerate()
        .map(|(i, item)| {
            let style = if i == app.selected && matches!(app.screen, Screen::Main) {
                Style::default()
                    .fg(t.accent)
                    .bg(t.highlight)
                    .add_modifier(Modifier::BOLD)
            } else {
                Style::default().fg(t.fg).bg(t.bg)
            };
            let prefix = if i == app.selected && matches!(app.screen, Screen::Main) { "▸ " } else { "  " };
            ListItem::new(Line::from(Span::styled(format!("{}{}", prefix, item), style)))
        })
        .collect();

    let list = List::new(items).block(
        Block::default()
            .borders(Borders::ALL)
            .border_style(Style::default().fg(t.border))
            .title("Menu")
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
            let settings_items: Vec<ListItem> = app
                .settings_items
                .iter()
                .enumerate()
                .map(|(i, item)| {
                    let style = if i == app.selected {
                        Style::default()
                            .fg(t.accent)
                            .add_modifier(Modifier::BOLD)
                    } else {
                        Style::default().fg(t.fg)
                    };
                    let prefix = if i == app.selected { "▸ " } else { "  " };
                    ListItem::new(Line::from(Span::styled(format!("{}{}", prefix, item), style)))
                })
                .collect();

            let settings_list = List::new(settings_items).block(
                Block::default()
                    .borders(Borders::ALL)
                    .border_style(Style::default().fg(t.border))
                    .title("Settings")
                    .title_style(Style::default().fg(t.title).add_modifier(Modifier::BOLD)),
            );
            f.render_widget(settings_list, area);
        }
        Screen::SettingsTheme => {
            let theme_items: Vec<ListItem> = Theme::names()
                .iter()
                .enumerate()
                .map(|(i, name)| {
                    let is_current = name == &app.theme.name;
                    let style = if i == app.selected {
                        Style::default()
                            .fg(t.accent)
                            .add_modifier(Modifier::BOLD)
                    } else {
                        Style::default().fg(t.fg)
                    };
                    let marker = if is_current { "●" } else { "○" };
                    let prefix = if i == app.selected { "▸ " } else { "  " };
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
            let networks = vec!["localnet", "devnet", "mainnet"];
            let network_items: Vec<ListItem> = networks
                .iter()
                .enumerate()
                .map(|(i, name)| {
                    let is_current = name == &app.network.as_str();
                    let style = if i == app.selected {
                        Style::default()
                            .fg(t.accent)
                            .add_modifier(Modifier::BOLD)
                    } else {
                        Style::default().fg(t.fg)
                    };
                    let marker = if is_current { "●" } else { "○" };
                    let prefix = if i == app.selected { "▸ " } else { "  " };
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
