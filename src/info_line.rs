use crate::app::MusicState;
use ratatui::{
    prelude::*,
    symbols::border,
    widgets::{block::*, *},
};

#[derive(Default)]
pub struct InfoLineWidget;

impl InfoLineWidget {
    pub fn render(self, area: Rect, buf: &mut Buffer, music_state: &MusicState) {
        let layout = Layout::default()
            .direction(Direction::Horizontal)
            .constraints(vec![Constraint::Percentage(70), Constraint::Percentage(30)])
            .split(area);

        let title = Title::from(format!("{}/{}", music_state.progress, music_state.duration));
        let block = Block::default()
            .title(title.alignment(Alignment::Center))
            .borders(Borders::ALL)
            .border_set(border::PLAIN);
        Widget::render(block, area, buf);
    }
}
