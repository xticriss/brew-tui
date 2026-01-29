use ratatui::{
    layout::{Constraint, Layout, Rect},
    style::{Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
    Frame,
};

use crate::app::{App, DependencyTreeMode};
use crate::ui::colors::Theme;

pub fn render(frame: &mut Frame, area: Rect, app: &App, theme: &Theme) {
    let data = match &app.dependency_tree_data {
        Some(data) => data,
        None => {
            // Show loading or empty state
            let block = Block::default()
                .title(" Dependencies ")
                .borders(Borders::ALL)
                .border_style(theme.border_style());

            let content = if app.loading {
                format!(" {} Loading...", app.spinner_char())
            } else {
                " No package selected".to_string()
            };

            let paragraph = Paragraph::new(content).block(block);
            frame.render_widget(paragraph, area);
            return;
        }
    };

    // Create layout: header, content, footer
    let _chunks = Layout::vertical([
        Constraint::Length(3), // Header with mode selector
        Constraint::Fill(1),   // Tree content
        Constraint::Length(2), // Footer with counts
    ])
    .split(area);

    // Title based on mode
    let title = match app.dependency_tree_mode {
        DependencyTreeMode::Dependencies => format!(" Dependencies of \"{}\" ", data.package_name),
        DependencyTreeMode::Dependents => format!(" Dependents of \"{}\" ", data.package_name),
    };

    let block = Block::default()
        .title(title)
        .borders(Borders::ALL)
        .border_style(theme.border_style());

    frame.render_widget(block, area);

    // Inner area (inside the border)
    let inner = Block::default().borders(Borders::ALL).inner(area);

    // Header: Mode selector
    let deps_style = if app.dependency_tree_mode == DependencyTreeMode::Dependencies {
        Style::default().fg(theme.title).add_modifier(Modifier::BOLD)
    } else {
        Style::default().fg(theme.fg)
    };

    let dependents_style = if app.dependency_tree_mode == DependencyTreeMode::Dependents {
        Style::default().fg(theme.title).add_modifier(Modifier::BOLD)
    } else {
        Style::default().fg(theme.fg)
    };

    let mode_line = Line::from(vec![
        Span::raw("  Mode: "),
        Span::styled("[", Style::default().fg(theme.fg)),
        Span::styled("Dependencies", deps_style),
        Span::styled("]", Style::default().fg(theme.fg)),
        Span::raw(" / "),
        Span::styled("[", Style::default().fg(theme.fg)),
        Span::styled("Dependents", dependents_style),
        Span::styled("]", Style::default().fg(theme.fg)),
        Span::raw("  (Tab to switch)"),
    ]);

    let header_area = Rect {
        x: inner.x,
        y: inner.y,
        width: inner.width,
        height: 1,
    };
    frame.render_widget(Paragraph::new(mode_line), header_area);

    // Content: Tree list
    let items = match app.dependency_tree_mode {
        DependencyTreeMode::Dependencies => &data.dependencies,
        DependencyTreeMode::Dependents => &data.dependents,
    };

    let content_area = Rect {
        x: inner.x,
        y: inner.y + 2,
        width: inner.width,
        height: inner.height.saturating_sub(4),
    };

    if items.is_empty() {
        let empty_msg = match app.dependency_tree_mode {
            DependencyTreeMode::Dependencies => "  No dependencies",
            DependencyTreeMode::Dependents => "  No packages depend on this",
        };
        let empty = Paragraph::new(empty_msg)
            .style(Style::default().fg(theme.fg).add_modifier(Modifier::ITALIC));
        frame.render_widget(empty, content_area);
    } else {
        let mut lines: Vec<Line> = Vec::new();
        let len = items.len();

        for (i, item) in items.iter().enumerate() {
            let prefix = if i == len - 1 { "  └─ " } else { "  ├─ " };
            lines.push(Line::from(vec![
                Span::styled(prefix, Style::default().fg(theme.border)),
                Span::styled(item.as_str(), Style::default().fg(theme.fg)),
            ]));
        }

        let tree = Paragraph::new(lines);
        frame.render_widget(tree, content_area);
    }

    // Footer: Counts
    let deps_count = data.dependencies.len();
    let dependents_count = data.dependents.len();

    let footer_line = Line::from(vec![
        Span::raw("  "),
        Span::styled(
            format!("{} dependencies", deps_count),
            Style::default().fg(theme.status_current),
        ),
        Span::raw(" │ "),
        Span::styled(
            format!("{} dependents", dependents_count),
            Style::default().fg(theme.status_pinned),
        ),
    ]);

    let footer_area = Rect {
        x: inner.x,
        y: inner.y + inner.height - 1,
        width: inner.width,
        height: 1,
    };
    frame.render_widget(Paragraph::new(footer_line), footer_area);
}
