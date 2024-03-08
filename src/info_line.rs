use crate::app::MusicState;
use ratatui::{
    prelude::*,
    symbols::border,
    widgets::{block::*, *},
};

#[derive(Default)]
pub struct InfoLineWidget;

impl InfoLineWidget {
    pub fn render(self, area: Rect, buf: &mut Buffer, music_state: Option<&MusicState>) {
        let layout = Layout::default()
            .direction(Direction::Horizontal)
            .constraints(vec![Constraint::Percentage(30), Constraint::Percentage(70)])
            .split(area);

        let block = Block::default()
            .borders(Borders::ALL)
            .border_set(border::PLAIN);
        Widget::render(block, area, buf);

        if let Some(m) = music_state {
            let progress_ratio = m.progress as f64 / m.duration as f64;
            let gauge = Gauge::default()
                .gauge_style(
                    Style::default()
                        .fg(Color::White)
                        .bg(Color::Black)
                        .add_modifier(Modifier::ITALIC),
                )
                .ratio(progress_ratio);

            Widget::render(gauge, layout[1], buf);
        }
    }
}
