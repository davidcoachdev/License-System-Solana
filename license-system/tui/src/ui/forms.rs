use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
    Frame,
};

use crate::app::{App, FormField, Theme};

pub fn render_form(f: &mut Frame, app: &App, area: Rect) {
    let t = &app.theme;
    
    let mut lines = Vec::new();
    
    for (i, field) in app.form_fields.iter().enumerate() {
        let is_active = i == app.form_index;
        
        let label_style = if is_active {
            Style::default().fg(t.accent).add_modifier(Modifier::BOLD)
        } else {
            Style::default().fg(t.muted)
        };
        
        let arrow = if is_active { "▸" } else { " " };
        lines.push(Line::from(Span::styled(
            format!("  {} {}", arrow, field.label),
            label_style,
        )));
        
        let bar = if is_active { "┃" } else { "│" };
        
        let value_text = if !field.options.is_empty() {
            if is_active {
                format!("{}  ◀ {} ▶", bar, field.value)
            } else {
                format!("{}  {}", bar, field.value)
            }
        } else if field.value.is_empty() {
            if is_active {
                format!("{}  | {}", bar, field.placeholder)
            } else {
                format!("{}  {}", bar, field.placeholder)
            }
        } else {
            if field.masked {
                format!("{}  {}", bar, "*".repeat(field.value.len()))
            } else {
                format!("{}  {}", bar, field.value)
            }
        };
        
        let value_style = if field.value.is_empty() {
            Style::default().fg(t.muted).bg(t.bg)
        } else {
            Style::default().fg(t.accent).bg(t.bg)
        };
        
        lines.push(Line::from(Span::styled(format!("    {}", value_text), value_style)));
        lines.push(Line::from(""));
    }
    
    lines.push(Line::from(""));
    lines.push(Line::from(Span::styled(
        "  ↑↓ Navigate fields  |  ←→ Change option  |  Enter Submit  |  ESC Cancel",
        Style::default().fg(t.muted),
    )));
    
    let form_block = Paragraph::new(lines)
        .style(Style::default().bg(t.bg))
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_style(Style::default().fg(t.border))
                .title("Form")
                .title_style(Style::default().fg(t.title).add_modifier(Modifier::BOLD)),
        );
    
    f.render_widget(form_block, area);
}
