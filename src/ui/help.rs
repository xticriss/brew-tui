use ratatui::{
    layout::{Constraint, Layout, Rect},
    style::{Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Clear, Paragraph},
    Frame,
};

use crate::ui::colors::Theme;
use crate::ui::constants::{HELP_POPUP_WIDTH, HELP_POPUP_HEIGHT};
use crate::ui::utils::centered_rect;

pub fn render(frame: &mut Frame, area: Rect, theme: &Theme) {
    // Create centered popup area
    let popup_area = centered_rect(HELP_POPUP_WIDTH, HELP_POPUP_HEIGHT, area);

    // Clear the background
    frame.render_widget(Clear, popup_area);

    let block = Block::default()
        .title(" Help - Press ? or Esc to close ")
        .title_style(theme.title_style())
        .borders(Borders::ALL)
        .border_style(theme.border_style())
        .style(Style::default().bg(theme.bg));

    let inner = block.inner(popup_area);
    frame.render_widget(block, popup_area);

    let chunks = Layout::vertical([
        Constraint::Length(6),  // Navigation
        Constraint::Length(8),  // Actions
        Constraint::Length(5),  // Tabs
        Constraint::Length(4),  // Search
        Constraint::Fill(1),    // Rest
    ])
    .split(inner);

    // Navigation section
    let navigation = create_section(
        "Navigation",
        vec![
            ("j / ↓", "Move down"),
            ("k / ↑", "Move up"),
            ("g / Home", "Go to first item"),
            ("G / End", "Go to last item"),
            ("Ctrl+d / PgDn", "Page down"),
            ("Ctrl+u / PgUp", "Page up"),
        ],
        theme,
    );
    frame.render_widget(navigation, chunks[0]);

    // Actions section
    let actions = create_section(
        "Actions",
        vec![
            ("o", "Toggle outdated filter"),
            ("u", "Update selected package"),
            ("U", "Update all outdated packages"),
            ("x / Del", "Remove selected package"),
            ("p", "Pin/unpin package"),
            ("c", "Cleanup old versions"),
            ("r", "Refresh package list"),
            ("Enter / d", "Toggle details panel"),
            ("t", "Toggle dependency tree"),
        ],
        theme,
    );
    frame.render_widget(actions, chunks[1]);

    // Tabs section
    let tabs = create_section(
        "Tabs",
        vec![
            ("Tab", "Next tab / Switch tree mode"),
            ("Shift+Tab", "Previous tab"),
            ("1", "Switch to Formulae"),
            ("2", "Switch to Casks"),
        ],
        theme,
    );
    frame.render_widget(tabs, chunks[2]);

    // Search section
    let search = create_section(
        "Search",
        vec![
            ("/", "Start search"),
            ("Esc / Enter", "Exit search"),
        ],
        theme,
    );
    frame.render_widget(search, chunks[3]);
}

fn create_section<'a>(title: &'a str, items: Vec<(&'a str, &'a str)>, theme: &Theme) -> Paragraph<'a> {
    let mut lines = vec![Line::from(Span::styled(
        format!(" {}", title),
        Style::default()
            .add_modifier(Modifier::BOLD | Modifier::UNDERLINED)
            .fg(theme.title),
    ))];

    for (key, desc) in items {
        lines.push(Line::from(vec![
            Span::raw("   "),
            Span::styled(
                format!("{:15}", key),
                Style::default().add_modifier(Modifier::BOLD),
            ),
            Span::raw(desc),
        ]));
    }

    Paragraph::new(lines)
}
