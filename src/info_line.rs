use crate::MusicState;
use ratatui::{
    prelude::*,
    symbols::border,
    widgets::{block::*, *},
};

#[derive(Default)]
pub struct InfoLineWidget;

impl InfoLineWidget {
    pub fn render(self, area: Rect, buf: &mut Buffer, _music_state: MusicState) {
        let block = Block::default()
            .borders(Borders::ALL)
            .border_set(border::PLAIN);
        Widget::render(block, area, buf);
    }
}
