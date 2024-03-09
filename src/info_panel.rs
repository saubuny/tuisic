use ratatui::{
    prelude::*,
    symbols::border,
    widgets::{block::*, *},
};

#[derive(Default)]
pub struct InfoPanelWidget;

impl InfoPanelWidget {
    pub fn render(self, area: Rect, buf: &mut Buffer, metadata: String) {
        let title = Title::from("Metadata".bold());
        let block = Block::default()
            .title(title.alignment(Alignment::Center))
            .borders(Borders::ALL)
            .border_set(border::PLAIN);
        let mut lines = vec![];

        let mut str = String::from("");
        for (_i, ch) in metadata.chars().enumerate() {
            str.push(ch);
            if ch == '\n' {
                lines.push(Line::from(str));
                str = String::from("");
            }
        }

        Paragraph::new(lines).block(block).render(area, buf);
    }
}
