use ratatui::{
    layout::Rect,
    style::{Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Tabs as RataTabs},
    Frame,
};

use crate::app::{App, Tab};
use crate::ui::colors::Theme;

pub fn render(frame: &mut Frame, area: Rect, app: &App, theme: &Theme) {
    let titles: Vec<Line> = vec![
        Line::from(vec![
            Span::raw(" "),
            Span::styled(
                "1",
                Style::default().add_modifier(Modifier::UNDERLINED),
            ),
            Span::raw(" Formulae "),
        ]),
        Line::from(vec![
            Span::raw(" "),
            Span::styled(
                "2",
                Style::default().add_modifier(Modifier::UNDERLINED),
            ),
            Span::raw(" Casks "),
        ]),
    ];

    let selected_index = match app.active_tab {
        Tab::Formulae => 0,
        Tab::Casks => 1,
    };

    let tabs = RataTabs::new(titles)
        .block(
            Block::default()
                .borders(Borders::BOTTOM)
                .border_style(theme.border_style())
                .title(" Brew TUI ")
                .title_style(theme.title_style()),
        )
        .select(selected_index)
        .style(theme.normal_style())
        .highlight_style(theme.tab_style(true))
        .divider("|");

    frame.render_widget(tabs, area);
}
