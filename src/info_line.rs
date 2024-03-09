use crate::app::MusicState;
use ratatui::{prelude::*, widgets::*};

#[derive(Default)]
pub struct InfoLineWidget;

impl InfoLineWidget {
    pub fn render(self, area: Rect, buf: &mut Buffer, music_state: MusicState) {
        let mut progress_ratio = 0.;
        if music_state.duration > 0 {
            progress_ratio = music_state.progress as f64 / music_state.duration as f64;
        }
        let progress = format!(
            "{:0>2}:{:0>2}",
            music_state.progress / 60,
            music_state.progress % 60
        );
        let duration = format!(
            "{:0>2}:{:0>2}",
            music_state.duration / 60,
            music_state.duration % 60
        );
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
