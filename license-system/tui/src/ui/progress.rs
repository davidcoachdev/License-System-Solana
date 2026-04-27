use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Modifier, Style},
    widgets::{Block, Borders, Clear, Gauge, Paragraph},
    Frame,
};

use crate::app::Theme;

pub struct ProgressModal {
    pub title: String,
    pub message: String,
    pub progress: u16,
}

impl ProgressModal {
    pub fn new(title: &str, message: &str, progress: u16) -> Self {
        Self {
            title: title.to_string(),
            message: message.to_string(),
            progress: progress.min(100),
        }
    }

    pub fn render(&self, f: &mut Frame, theme: &Theme) {
        let area = centered_rect(50, 25, f.area());
        
        f.render_widget(Clear, area);
        
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(3),
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
                    .border_style(Style::default().fg(theme.accent))
                    .title(self.title.as_str())
                    .title_style(Style::default().fg(theme.title).add_modifier(Modifier::BOLD)),
            );
        f.render_widget(message_block, chunks[0]);

        let gauge = Gauge::default()
            .block(Block::default().borders(Borders::ALL).border_style(Style::default().fg(theme.border)))
            .gauge_style(Style::default().fg(theme.success).bg(theme.bg))
            .percent(self.progress);
        f.render_widget(gauge, chunks[1]);
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
