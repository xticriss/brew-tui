use ratatui::{
    layout::{Alignment, Constraint, Layout, Rect},
    style::{Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Clear, Paragraph},
    Frame,
};

use crate::events::ConfirmAction;
use crate::ui::colors::Theme;
use crate::ui::constants::{CONFIRM_POPUP_WIDTH, CONFIRM_POPUP_HEIGHT};
use crate::ui::utils::centered_rect;

pub fn render(frame: &mut Frame, area: Rect, action: &ConfirmAction, theme: &Theme) {
    // Create centered popup area
    let popup_area = centered_rect(CONFIRM_POPUP_WIDTH, CONFIRM_POPUP_HEIGHT, area);

    // Clear the background
    frame.render_widget(Clear, popup_area);

    let block = Block::default()
        .title(" Confirm ")
        .title_style(theme.title_style())
        .borders(Borders::ALL)
        .border_style(theme.border_style())
        .style(Style::default().bg(theme.bg));

    let inner = block.inner(popup_area);
    frame.render_widget(block, popup_area);

    let chunks = Layout::vertical([
        Constraint::Fill(1),
        Constraint::Length(1),
        Constraint::Length(2),
        Constraint::Fill(1),
    ])
    .split(inner);

    // Message
    let message = Paragraph::new(action.message())
        .alignment(Alignment::Center)
        .style(Style::default().add_modifier(Modifier::BOLD));
    frame.render_widget(message, chunks[1]);

    // Buttons
    let buttons = Line::from(vec![
        Span::styled(
            " [Y]es ",
            Style::default()
                .fg(theme.success)
                .add_modifier(Modifier::BOLD),
        ),
        Span::raw("  "),
        Span::styled(
            " [N]o ",
            Style::default()
                .fg(theme.error)
                .add_modifier(Modifier::BOLD),
        ),
    ]);
    let buttons_widget = Paragraph::new(buttons).alignment(Alignment::Center);
    frame.render_widget(buttons_widget, chunks[2]);
}
