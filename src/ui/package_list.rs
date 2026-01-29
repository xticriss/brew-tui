use ratatui::{
    layout::{Constraint, Rect},
    style::{Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Cell, Row, Scrollbar, ScrollbarOrientation, ScrollbarState, Table},
    Frame,
};

use crate::app::{App, Tab};
use crate::brew::types::PackageDisplay;
use crate::ui::colors::Theme;
use crate::ui::constants::*;

pub fn render(frame: &mut Frame, area: Rect, app: &mut App, theme: &Theme) {
    let header = Row::new(vec![
        Cell::from("Name"),
        Cell::from("Version"),
        Cell::from("Status"),
        Cell::from("Description"),
    ])
    .style(theme.header_style())
    .height(1);

    // Lowercase filter once for all highlight_match calls
    let filter_lower = app.filter.to_lowercase();
    let active_tab = app.active_tab.clone();

    // Build rows based on active tab
    let rows: Vec<Row> = match active_tab {
        Tab::Formulae => {
            let packages = app.filtered_packages();
            packages
                .iter()
                .enumerate()
                .map(|(i, pkg)| {
                    let style = if i % 2 == 0 {
                        theme.normal_style()
                    } else {
                        theme.alt_row_style()
                    };

                    let status_style = theme.status_style(pkg.status_display());
                    let status_symbol = match pkg.status_display() {
                        STATUS_CURRENT => SYMBOL_CURRENT,
                        STATUS_OUTDATED => SYMBOL_OUTDATED,
                        STATUS_PINNED => SYMBOL_PINNED,
                        _ => "",
                    };

                    Row::new(vec![
                        Cell::from(highlight_match(&pkg.name, &filter_lower, theme)),
                        Cell::from(pkg.display_version().to_string()),
                        Cell::from(Span::styled(
                            format!("{} {}", status_symbol, pkg.status_display()),
                            status_style,
                        )),
                        Cell::from(truncate_string(pkg.display_description(), DESC_TRUNCATE_LEN)),
                    ])
                    .style(style)
                    .height(1)
                })
                .collect()
        }
        Tab::Casks => {
            let casks = app.filtered_casks();
            casks
                .iter()
                .enumerate()
                .map(|(i, cask)| {
                    let style = if i % 2 == 0 {
                        theme.normal_style()
                    } else {
                        theme.alt_row_style()
                    };

                    let status_style = theme.status_style(cask.status_display());
                    let status_symbol = match cask.status_display() {
                        STATUS_CURRENT => SYMBOL_CURRENT,
                        STATUS_OUTDATED => SYMBOL_OUTDATED,
                        _ => "",
                    };

                    Row::new(vec![
                        Cell::from(highlight_match(cask.display_name(), &filter_lower, theme)),
                        Cell::from(cask.display_version().to_string()),
                        Cell::from(Span::styled(
                            format!("{} {}", status_symbol, cask.status_display()),
                            status_style,
                        )),
                        Cell::from(truncate_string(cask.display_description(), DESC_TRUNCATE_LEN)),
                    ])
                    .style(style)
                    .height(1)
                })
                .collect()
        }
    };

    let total_items = rows.len();

    let widths = [
        Constraint::Length(NAME_COLUMN_WIDTH),
        Constraint::Length(VERSION_COLUMN_WIDTH),
        Constraint::Length(STATUS_COLUMN_WIDTH),
        Constraint::Fill(1),
    ];

    let table = Table::new(rows, widths)
        .header(header)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_style(theme.border_style()),
        )
        .row_highlight_style(theme.selected_style())
        .highlight_symbol("► ");

    // Now get mutable reference to state after rows are built
    let list_state = match active_tab {
        Tab::Formulae => &mut app.formulae_state,
        Tab::Casks => &mut app.casks_state,
    };

    frame.render_stateful_widget(table, area, list_state);

    // Render scrollbar
    if total_items > 0 {
        let selected = list_state.selected().unwrap_or(0);
        let mut scrollbar_state = ScrollbarState::new(total_items).position(selected);

        let scrollbar = Scrollbar::default()
            .orientation(ScrollbarOrientation::VerticalRight)
            .begin_symbol(Some("▲"))
            .end_symbol(Some("▼"));

        frame.render_stateful_widget(
            scrollbar,
            area.inner(ratatui::layout::Margin {
                vertical: 1,
                horizontal: 0,
            }),
            &mut scrollbar_state,
        );
    }
}

/// Optimized highlight with pre-lowercased filter to reduce allocations
fn highlight_match(text: &str, filter_lower: &str, theme: &Theme) -> Line<'static> {
    if filter_lower.is_empty() {
        return Line::from(text.to_string());
    }

    let text_lower = text.to_lowercase();

    if let Some(start) = text_lower.find(filter_lower) {
        let end = start + filter_lower.len();

        // Pre-allocate with known capacity
        let mut spans = Vec::with_capacity(3);

        if start > 0 {
            spans.push(Span::raw(text[..start].to_string()));
        }
        spans.push(Span::styled(
            text[start..end].to_string(),
            Style::default()
                .fg(theme.search_highlight)
                .add_modifier(Modifier::BOLD)
        ));
        if end < text.len() {
            spans.push(Span::raw(text[end..].to_string()));
        }

        Line::from(spans)
    } else {
        Line::from(text.to_string())
    }
}

/// Safely truncates a string at UTF-8 character boundaries
/// Prevents panics on emoji, CJK, and multi-byte characters
fn truncate_string(s: &str, max_len: usize) -> String {
    if s.len() <= max_len {
        return s.to_string();
    }

    // Find safe truncation point (UTF-8 char boundary)
    let mut end = max_len.saturating_sub(1);
    while end > 0 && !s.is_char_boundary(end) {
        end -= 1;
    }

    if end == 0 {
        String::new()
    } else {
        format!("{}…", &s[..end])
    }
}
