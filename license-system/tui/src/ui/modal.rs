use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Clear, Paragraph},
    Frame,
};

use crate::app::Theme;

pub struct ConfirmModal {
    pub title: String,
    pub message: String,
    pub confirm_text: String,
    pub cancel_text: String,
    pub selected: bool,
}

impl ConfirmModal {
    pub fn new(title: &str, message: &str) -> Self {
        Self {
            title: title.to_string(),
            message: message.to_string(),
            confirm_text: "Yes".to_string(),
            cancel_text: "No".to_string(),
            selected: false,
        }
    }

    pub fn with_buttons(mut self, confirm: &str, cancel: &str) -> Self {
        self.confirm_text = confirm.to_string();
        self.cancel_text = cancel.to_string();
        self
    }

    pub fn toggle_selection(&mut self) {
        self.selected = !self.selected;
    }

    pub fn is_confirm_selected(&self) -> bool {
        self.selected
    }

    pub fn render(&self, f: &mut Frame, theme: &Theme) {
        let area = centered_rect(50, 30, f.area());
        
        f.render_widget(Clear, area);
        
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Min(3),
                Constraint::Length(3),
            ])
            .split(area);

        let message_block = Paragraph::new(self.message.as_str())
            .alignment(Alignment::Center)
            .style(Style::default().fg(theme.fg).bg(theme.bg))
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .border_style(Style::default().fg(theme.accent))
                    .title(self.title.as_str())
                    .title_style(Style::default().fg(theme.title).add_modifier(Modifier::BOLD)),
            );
        f.render_widget(message_block, chunks[0]);

        let button_chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Percentage(50),
                Constraint::Percentage(50),
            ])
            .split(chunks[1]);

        let cancel_style = if !self.selected {
            Style::default()
                .fg(theme.bg)
                .bg(theme.accent)
                .add_modifier(Modifier::BOLD)
        } else {
            Style::default().fg(theme.fg).bg(theme.bg)
        };

        let confirm_style = if self.selected {
            Style::default()
                .fg(theme.bg)
                .bg(theme.error)
                .add_modifier(Modifier::BOLD)
        } else {
            Style::default().fg(theme.fg).bg(theme.bg)
        };

        let cancel_button = Paragraph::new(self.cancel_text.as_str())
            .alignment(Alignment::Center)
            .style(cancel_style)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .border_style(Style::default().fg(theme.border)),
            );
        f.render_widget(cancel_button, button_chunks[0]);

        let confirm_button = Paragraph::new(self.confirm_text.as_str())
            .alignment(Alignment::Center)
            .style(confirm_style)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .border_style(Style::default().fg(theme.border)),
            );
        f.render_widget(confirm_button, button_chunks[1]);
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
