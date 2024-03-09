use crate::app::MusicState;
use ratatui::{
    prelude::*,
    symbols::border,
    widgets::{block::*, *},
};

#[derive(Default)]
pub struct InfoLineWidget;

impl InfoLineWidget {
    pub fn render(self, area: Rect, buf: &mut Buffer, m: MusicState) {
        let mut progress_ratio = 0.;
        if m.duration > 0 {
            progress_ratio = m.progress as f64 / m.duration as f64;
        }
        let progress = format!("{:0>2}:{:0>2}", m.progress / 60, m.progress % 60);
        let duration = format!("{:0>2}:{:0>2}", m.duration / 60, m.duration % 60);
        let label = format!("{}/{}", progress, duration);
        let gauge = LineGauge::default()
            .gauge_style(
                Style::default()
                    .fg(Color::White)
                    .bg(Color::Black)
                    .add_modifier(Modifier::BOLD),
            )
            .label(label)
            .line_set(symbols::line::THICK)
            .ratio(progress_ratio);

        Widget::render(gauge, area, buf);
    }
}
