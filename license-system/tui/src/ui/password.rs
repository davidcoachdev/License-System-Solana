use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Clear, Paragraph},
    Frame,
};

use crate::app::Theme;

pub struct PasswordModal {
    pub title: String,
    pub message: String,
    pub password: String,
}

impl PasswordModal {
    pub fn new(title: &str, message: &str) -> Self {
        Self {
            title: title.to_string(),
            message: message.to_string(),
            password: String::new(),
        }
    }

    pub fn render(&self, f: &mut Frame, theme: &Theme) {
        let area = centered_rect(50, 30, f.area());
        
        f.render_widget(Clear, area);
        
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(5),
                Constraint::Length(3),
                Constraint::Min(1),
            ])
            .split(area);

        let message_block = Paragraph::new(self.message.as_str())
            .alignment(Alignment::Center)
            .style(Style::default().fg(theme.fg).bg(theme.bg))
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .border_style(Style::default().fg(theme.warning))
                    .title(self.title.as_str())
                    .title_style(Style::default().fg(theme.warning).add_modifier(Modifier::BOLD)),
            );
        f.render_widget(message_block, chunks[0]);

        let password_text = Line::from(vec![
            Span::styled("  Password: ", Style::default().fg(theme.fg)),
            Span::styled("*".repeat(self.password.len()), Style::default().fg(theme.accent)),
        ]);

        let password_block = Paragraph::new(password_text)
            .style(Style::default().bg(theme.bg))
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .border_style(Style::default().fg(theme.border)),
            );
        f.render_widget(password_block, chunks[1]);

        let help_text = "Type password and press Enter\nPress ESC to cancel";
        let help = Paragraph::new(help_text)
            .alignment(Alignment::Center)
            .style(Style::default().fg(theme.muted).bg(theme.bg));
        f.render_widget(help, chunks[2]);
    }
}

fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
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
