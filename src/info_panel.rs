use ratatui::{
    prelude::*,
    symbols::border,
    widgets::{block::*, *},
};

#[derive(Default)]
pub struct InfoPanelWidget;

impl InfoPanelWidget {
    pub fn render(self, area: Rect, buf: &mut Buffer) {
        let title = Title::from("Metadata".bold());
        let block = Block::default()
            .borders(Borders::ALL)
            .title(title)
            .border_set(border::PLAIN);

        let mut lines = vec![];

        Paragraph::new(lines).block(block).render(area, buf);
    }
}
