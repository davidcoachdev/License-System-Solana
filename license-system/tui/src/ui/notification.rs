use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Modifier, Style},
    text::Span,
    widgets::{Block, Borders, Clear, Paragraph},
    Frame,
};

use crate::app::Theme;

#[derive(Debug, Clone)]
pub enum NotificationType {
    Success,
    Error,
    Info,
    Warning,
}

pub struct NotificationModal {
    pub title: String,
    pub message: String,
    pub notification_type: NotificationType,
}

impl NotificationModal {
    pub fn success(title: &str, message: &str) -> Self {
        Self {
            title: format!("✅ {}", title),
            message: message.to_string(),
            notification_type: NotificationType::Success,
        }
    }

    pub fn error(title: &str, message: &str) -> Self {
        Self {
            title: format!("❌ {}", title),
            message: message.to_string(),
            notification_type: NotificationType::Error,
        }
    }

    pub fn info(title: &str, message: &str) -> Self {
        Self {
            title: format!("ℹ️  {}", title),
            message: message.to_string(),
            notification_type: NotificationType::Info,
        }
    }

    pub fn warning(title: &str, message: &str) -> Self {
        Self {
            title: format!("⚠️  {}", title),
            message: message.to_string(),
            notification_type: NotificationType::Warning,
        }
    }

    pub fn render(&self, f: &mut Frame, theme: &Theme) {
        let area = centered_rect(50, 30, f.area());
        
        f.render_widget(Clear, area);
        
        let color = match self.notification_type {
            NotificationType::Success => theme.success,
            NotificationType::Error => theme.error,
            NotificationType::Info => theme.accent,
            NotificationType::Warning => theme.warning,
        };
        
        let message_block = Paragraph::new(self.message.as_str())
            .alignment(Alignment::Center)
            .style(Style::default().fg(theme.fg).bg(theme.bg))
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .border_style(Style::default().fg(color))
                    .title(self.title.as_str())
                    .title_style(Style::default().fg(color).add_modifier(Modifier::BOLD)),
            );
        f.render_widget(message_block, area);
        
        let button_area = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Min(1),
                Constraint::Length(3),
            ])
            .split(area)[1];
        
        let button = Paragraph::new("Press any key to continue")
            .alignment(Alignment::Center)
            .style(Style::default().fg(theme.muted).bg(theme.bg));
        f.render_widget(button, button_area);
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
