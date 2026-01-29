use ratatui::{
    layout::{Constraint, Layout, Rect},
    style::{Modifier, Style},
    text::{Line, Span},
    widgets::Paragraph,
    Frame,
};

use crate::app::{App, InputMode, Tab};
use crate::ui::colors::Theme;

pub fn render(frame: &mut Frame, area: Rect, app: &App, theme: &Theme) {
    let chunks = Layout::horizontal([
        Constraint::Percentage(40),
        Constraint::Percentage(30),
        Constraint::Percentage(30),
    ])
    .split(area);

    // Left section: Package counts
    let (total, outdated) = match app.active_tab {
        Tab::Formulae => {
            let outdated = app.packages.iter().filter(|p| p.outdated).count();
            (app.packages.len(), outdated)
        }
        Tab::Casks => {
            let outdated = app.casks.iter().filter(|c| c.outdated).count();
            (app.casks.len(), outdated)
        }
    };

    let mut count_spans = vec![
        Span::raw(" "),
        Span::styled(
            format!("{}", total),
            Style::default().add_modifier(Modifier::BOLD),
        ),
        Span::raw(" packages"),
        Span::raw(" │ "),
        Span::styled(
            format!("{}", outdated),
            Style::default()
                .fg(if outdated > 0 {
                    theme.status_outdated
                } else {
                    theme.status_current
                })
                .add_modifier(Modifier::BOLD),
        ),
        Span::raw(" outdated"),
    ];

    // Add filter indicator if outdated filter is active
    if app.show_outdated_only {
        count_spans.push(Span::raw(" │ "));
        count_spans.push(Span::styled(
            "⚡ OUTDATED ONLY",
            Style::default()
                .fg(theme.status_outdated)
                .add_modifier(Modifier::BOLD),
        ));
    }

    let counts = Line::from(count_spans);
    frame.render_widget(Paragraph::new(counts), chunks[0]);

    // Center section: Status message or search
    let center_content = match &app.input_mode {
        InputMode::Search => {
            Line::from(vec![
                Span::styled(" / ", Style::default().fg(theme.search_highlight)),
                Span::raw(&app.filter),
                Span::styled("█", Style::default().add_modifier(Modifier::SLOW_BLINK)),
            ])
        }
        _ => {
            if app.loading {
                // Show animated spinner with status message
                let spinner = app.spinner_char();
                let msg = app.status_message.as_deref().unwrap_or("Working...");
                Line::from(vec![
                    Span::styled(
                        format!(" {} ", spinner),
                        Style::default().fg(theme.title).add_modifier(Modifier::BOLD),
                    ),
                    Span::styled(msg, Style::default().fg(theme.title)),
                ])
            } else if let Some(msg) = &app.status_message {
                let color = if msg.starts_with("Error") {
                    theme.error
                } else {
                    theme.success
                };
                Line::from(Span::styled(format!(" {}", msg), Style::default().fg(color)))
            } else {
                Line::from("")
            }
        }
    };
    frame.render_widget(Paragraph::new(center_content), chunks[1]);

    // Right section: Help hint
    let help = Line::from(vec![
        Span::styled("?", Style::default().add_modifier(Modifier::BOLD)),
        Span::raw(": help │ "),
        Span::styled("q", Style::default().add_modifier(Modifier::BOLD)),
        Span::raw(": quit "),
    ]);
    frame.render_widget(Paragraph::new(help).right_aligned(), chunks[2]);
}
