use ratatui::style::{Color, Modifier, Style};

pub struct Theme {
    pub bg: Color,
    pub fg: Color,
    pub highlight_bg: Color,
    pub highlight_fg: Color,
    pub border: Color,
    pub title: Color,
    pub status_current: Color,
    pub status_outdated: Color,
    pub status_pinned: Color,
    pub header_bg: Color,
    pub header_fg: Color,
    pub alt_row_bg: Color,
    pub search_highlight: Color,
    pub error: Color,
    pub success: Color,
}

impl Default for Theme {
    fn default() -> Self {
        Self {
            bg: Color::Reset,
            fg: Color::White,
            highlight_bg: Color::Rgb(60, 60, 80),
            highlight_fg: Color::White,
            border: Color::Rgb(100, 100, 120),
            title: Color::Cyan,
            status_current: Color::Green,
            status_outdated: Color::Yellow,
            status_pinned: Color::Blue,
            header_bg: Color::Rgb(40, 40, 60),
            header_fg: Color::White,
            alt_row_bg: Color::Rgb(30, 30, 40),
            search_highlight: Color::Magenta,
            error: Color::Red,
            success: Color::Green,
        }
    }
}

impl Theme {
    pub fn header_style(&self) -> Style {
        Style::default()
            .fg(self.header_fg)
            .bg(self.header_bg)
            .add_modifier(Modifier::BOLD)
    }

    pub fn selected_style(&self) -> Style {
        Style::default()
            .fg(self.highlight_fg)
            .bg(self.highlight_bg)
            .add_modifier(Modifier::BOLD)
    }

    pub fn normal_style(&self) -> Style {
        Style::default().fg(self.fg).bg(self.bg)
    }

    pub fn alt_row_style(&self) -> Style {
        Style::default().fg(self.fg).bg(self.alt_row_bg)
    }

    pub fn border_style(&self) -> Style {
        Style::default().fg(self.border)
    }

    pub fn title_style(&self) -> Style {
        Style::default().fg(self.title).add_modifier(Modifier::BOLD)
    }

    pub fn status_style(&self, status: &str) -> Style {
        let color = match status {
            "Current" => self.status_current,
            "Outdated" => self.status_outdated,
            "Pinned" => self.status_pinned,
            _ => self.fg,
        };
        Style::default().fg(color)
    }

    pub fn tab_style(&self, selected: bool) -> Style {
        if selected {
            Style::default()
                .fg(self.title)
                .add_modifier(Modifier::BOLD | Modifier::UNDERLINED)
        } else {
            Style::default().fg(self.fg)
        }
    }
}
