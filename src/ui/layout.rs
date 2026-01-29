use ratatui::{
    layout::{Constraint, Layout},
    Frame,
};

use crate::app::{App, InputMode};
use crate::ui::{colors::Theme, confirm, dependency_tree, details, help, package_list, status_bar, tabs};

pub fn render(frame: &mut Frame, app: &mut App) {
    let theme = Theme::default();
    let area = frame.area();

    // Main layout: tabs | content | status bar
    let main_chunks = Layout::vertical([
        Constraint::Length(3),  // Tabs
        Constraint::Fill(1),    // Content
        Constraint::Length(1),  // Status bar
    ])
    .split(area);

    // Render tabs
    tabs::render(frame, main_chunks[0], app, &theme);

    // Content area: package list (with optional details or dependency tree panel)
    if app.show_details {
        let content_chunks = Layout::horizontal([
            Constraint::Percentage(60), // Package list
            Constraint::Percentage(40), // Details panel
        ])
        .split(main_chunks[1]);

        package_list::render(frame, content_chunks[0], app, &theme);
        details::render(frame, content_chunks[1], app, &theme);
    } else if app.show_dependency_tree {
        let content_chunks = Layout::horizontal([
            Constraint::Percentage(55), // Package list
            Constraint::Percentage(45), // Dependency tree panel
        ])
        .split(main_chunks[1]);

        package_list::render(frame, content_chunks[0], app, &theme);
        dependency_tree::render(frame, content_chunks[1], app, &theme);
    } else {
        package_list::render(frame, main_chunks[1], app, &theme);
    }

    // Status bar
    status_bar::render(frame, main_chunks[2], app, &theme);

    // Overlays (confirm takes priority over help)
    if let InputMode::Confirm = app.input_mode {
        if let Some(action) = &app.pending_action {
            confirm::render(frame, area, action, &theme);
        }
    } else if app.show_help {
        help::render(frame, area, &theme);
    }
}
