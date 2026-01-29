use ratatui::{
    layout::{Constraint, Layout, Rect},
    style::{Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph, Wrap},
    Frame,
};

use crate::app::{App, Tab};
use crate::ui::colors::Theme;

pub fn render(frame: &mut Frame, area: Rect, app: &App, theme: &Theme) {
    let block = Block::default()
        .title(" Package Details ")
        .title_style(theme.title_style())
        .borders(Borders::ALL)
        .border_style(theme.border_style());

    if let Some(info) = &app.selected_package_info {
        let inner_area = block.inner(area);
        frame.render_widget(block, area);

        let chunks = Layout::vertical([
            Constraint::Length(2), // Name & version
            Constraint::Length(1), // Status
            Constraint::Length(4), // Description
            Constraint::Length(2), // Homepage
            Constraint::Length(4), // Dependencies
            Constraint::Length(4), // Dependents
            Constraint::Fill(1),   // Remaining space
        ])
        .split(inner_area);

        // Name and version
        let name_line = Line::from(vec![
            Span::styled(&info.name, Style::default().add_modifier(Modifier::BOLD)),
            Span::raw(" "),
            Span::styled(
                format!("v{}", info.version),
                Style::default().fg(theme.border),
            ),
        ]);
        frame.render_widget(Paragraph::new(name_line), chunks[0]);

        // Status
        let status = if info.pinned {
            Span::styled("📌 Pinned", Style::default().fg(theme.status_pinned))
        } else if info.outdated {
            Span::styled("↑ Outdated", Style::default().fg(theme.status_outdated))
        } else {
            Span::styled("✓ Current", Style::default().fg(theme.status_current))
        };
        frame.render_widget(Paragraph::new(Line::from(status)), chunks[1]);

        // Description
        let desc_block = Block::default()
            .title("Description")
            .borders(Borders::NONE);
        let description = Paragraph::new(&*info.description)
            .block(desc_block)
            .wrap(Wrap { trim: true });
        frame.render_widget(description, chunks[2]);

        // Homepage
        if !info.homepage.is_empty() {
            let homepage = Line::from(vec![
                Span::styled("Homepage: ", Style::default().add_modifier(Modifier::BOLD)),
                Span::styled(&info.homepage, Style::default().fg(theme.title)),
            ]);
            frame.render_widget(Paragraph::new(homepage), chunks[3]);
        }

        // Dependencies
        let deps_title = format!("Dependencies ({})", info.dependencies.len());
        let deps_text = if info.dependencies.is_empty() {
            "None".to_string()
        } else {
            info.dependencies.join(", ")
        };
        let deps = Paragraph::new(vec![
            Line::from(Span::styled(
                deps_title,
                Style::default().add_modifier(Modifier::BOLD),
            )),
            Line::from(deps_text),
        ])
        .wrap(Wrap { trim: true });
        frame.render_widget(deps, chunks[4]);

        // Dependents (what uses this package)
        let dependents_title = format!("Used by ({})", info.dependents.len());
        let dependents_text = if info.dependents.is_empty() {
            "Nothing".to_string()
        } else {
            info.dependents.join(", ")
        };
        let dependents = Paragraph::new(vec![
            Line::from(Span::styled(
                dependents_title,
                Style::default().add_modifier(Modifier::BOLD),
            )),
            Line::from(dependents_text),
        ])
        .wrap(Wrap { trim: true });
        frame.render_widget(dependents, chunks[5]);
    } else {
        // No package selected
        let message = match app.active_tab {
            Tab::Formulae if app.packages.is_empty() => "No formulae installed",
            Tab::Casks if app.casks.is_empty() => "No casks installed",
            _ => "Press Enter or 'd' to view details",
        };
        let content = Paragraph::new(message)
            .block(block)
            .style(Style::default().add_modifier(Modifier::ITALIC));
        frame.render_widget(content, area);
    }
}
