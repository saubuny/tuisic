use ratatui::{
    prelude::*,
    symbols::border,
    widgets::{block::*, *},
};

#[derive(Default)]
pub struct MusicListWidget;

// TODO: Convert this to use a List instead of just paragraph lines
// TODO: Create a queue for music
// TODO: "/" for seaching (perhaps a Search mode on the main App state)
// TODO: Add controls as bottom title
impl MusicListWidget {
    pub fn render(self, area: Rect, buf: &mut Buffer, scroll_state: u16) {
        let title = Title::from("Files".bold());
        let block = Block::default()
            .title(title.alignment(Alignment::Center))
            .borders(Borders::ALL)
            .border_set(border::PLAIN);

        // TODO: Style current playing song green
        let lines = vec![
            Line::from(vec!["music".into()]),
            Line::from(vec!["music".into()]),
            Line::from(vec!["music".into()]),
            Line::from(vec!["music".into()]),
            Line::from(vec!["music".into()]),
            Line::from(vec!["music".into()]),
            Line::from(vec!["music".into()]),
        ];

        Paragraph::new(Text::from(lines.clone()))
            .left_aligned()
            .scroll((scroll_state, 0))
            .wrap(Wrap { trim: false })
            .block(block)
            .render(area, buf);

        let scrollbar = Scrollbar::new(ScrollbarOrientation::VerticalRight)
            .begin_symbol(Some("↑"))
            .end_symbol(Some("↓"));

        let mut scrollbar_state = ScrollbarState::new(lines.len()).position(scroll_state.into());

        StatefulWidget::render(
            scrollbar,
            area.inner(&Margin {
                vertical: 1,
                horizontal: 0,
            }),
            buf,
            &mut scrollbar_state,
        );
    }
}
