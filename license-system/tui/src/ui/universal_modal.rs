use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Clear, Gauge, Paragraph},
    Frame,
};

use crate::app::Theme;

#[derive(Debug, Clone)]
pub enum ModalType {
    Confirm { confirm_text: String, cancel_text: String, selected: bool },
    Notification { notification_type: NotificationType },
    Progress { progress: u16 },
    Password { password: String },
}

#[derive(Debug, Clone)]
pub enum NotificationType {
    Success,
    Error,
    Info,
    Warning,
}

pub struct Modal {
    pub title: String,
    pub message: String,
    pub modal_type: ModalType,
}

impl Modal {
    pub fn confirm(title: &str, message: &str) -> Self {
        Self {
            title: title.to_string(),
            message: message.to_string(),
            modal_type: ModalType::Confirm {
                confirm_text: "Yes".to_string(),
                cancel_text: "No".to_string(),
                selected: false,
            },
        }
    }

    pub fn success(title: &str, message: &str) -> Self {
        Self {
            title: format!("✅ {}", title),
            message: message.to_string(),
            modal_type: ModalType::Notification {
                notification_type: NotificationType::Success,
            },
        }
    }

    pub fn error(title: &str, message: &str) -> Self {
        Self {
            title: format!("❌ {}", title),
            message: message.to_string(),
            modal_type: ModalType::Notification {
                notification_type: NotificationType::Error,
            },
        }
    }

    pub fn info(title: &str, message: &str) -> Self {
        Self {
            title: format!("ℹ️  {}", title),
            message: message.to_string(),
            modal_type: ModalType::Notification {
                notification_type: NotificationType::Info,
            },
        }
    }

    pub fn warning(title: &str, message: &str) -> Self {
        Self {
            title: format!("⚠️  {}", title),
            message: message.to_string(),
            modal_type: ModalType::Notification {
                notification_type: NotificationType::Warning,
            },
        }
    }

    pub fn progress(title: &str, message: &str, progress: u16) -> Self {
        Self {
            title: title.to_string(),
            message: message.to_string(),
            modal_type: ModalType::Progress {
                progress: progress.min(100),
            },
        }
    }

    pub fn password(title: &str, message: &str) -> Self {
        Self {
            title: title.to_string(),
            message: message.to_string(),
            modal_type: ModalType::Password {
                password: String::new(),
            },
        }
    }

    pub fn toggle_confirm_selection(&mut self) {
        if let ModalType::Confirm { selected, .. } = &mut self.modal_type {
            *selected = !*selected;
        }
    }

    pub fn is_confirm_selected(&self) -> bool {
        if let ModalType::Confirm { selected, .. } = &self.modal_type {
            *selected
        } else {
            false
        }
    }

    pub fn push_password_char(&mut self, c: char) {
        if let ModalType::Password { password } = &mut self.modal_type {
            password.push(c);
        }
    }

    pub fn pop_password_char(&mut self) {
        if let ModalType::Password { password } = &mut self.modal_type {
            password.pop();
        }
    }

    pub fn get_password(&self) -> Option<String> {
        if let ModalType::Password { password } = &self.modal_type {
            Some(password.clone())
        } else {
            None
        }
    }

    pub fn render(&self, f: &mut Frame, theme: &Theme) {
        match &self.modal_type {
            ModalType::Confirm { confirm_text, cancel_text, selected } => {
                self.render_confirm(f, theme, confirm_text, cancel_text, *selected);
            }
            ModalType::Notification { notification_type } => {
                self.render_notification(f, theme, notification_type);
            }
            ModalType::Progress { progress } => {
                self.render_progress(f, theme, *progress);
            }
            ModalType::Password { password } => {
                self.render_password(f, theme, password);
            }
        }
    }

    fn render_notification(&self, f: &mut Frame, theme: &Theme, notification_type: &NotificationType) {
        let area = centered_rect(50, 30, f.area());
        f.render_widget(Clear, area);
        
        let color = match notification_type {
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
    }

    fn render_confirm(&self, f: &mut Frame, theme: &Theme, confirm_text: &str, cancel_text: &str, selected: bool) {
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

        let cancel_style = if !selected {
            Style::default()
                .fg(theme.bg)
                .bg(theme.accent)
                .add_modifier(Modifier::BOLD)
        } else {
            Style::default().fg(theme.fg).bg(theme.bg)
        };

        let confirm_style = if selected {
            Style::default()
                .fg(theme.bg)
                .bg(theme.error)
                .add_modifier(Modifier::BOLD)
        } else {
            Style::default().fg(theme.fg).bg(theme.bg)
        };

        let cancel_button = Paragraph::new(cancel_text)
            .alignment(Alignment::Center)
            .style(cancel_style)
            .block(Block::default().borders(Borders::ALL).border_style(Style::default().fg(theme.border)));
        f.render_widget(cancel_button, button_chunks[0]);

        let confirm_button = Paragraph::new(confirm_text)
            .alignment(Alignment::Center)
            .style(confirm_style)
            .block(Block::default().borders(Borders::ALL).border_style(Style::default().fg(theme.border)));
        f.render_widget(confirm_button, button_chunks[1]);
    }

    fn render_progress(&self, f: &mut Frame, theme: &Theme, progress: u16) {
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
            .percent(progress);
        f.render_widget(gauge, chunks[1]);
    }

    fn render_password(&self, f: &mut Frame, theme: &Theme, password: &str) {
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
            Span::styled("*".repeat(password.len()), Style::default().fg(theme.accent)),
        ]);

        let password_block = Paragraph::new(password_text)
            .style(Style::default().bg(theme.bg))
            .block(Block::default().borders(Borders::ALL).border_style(Style::default().fg(theme.border)));
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
