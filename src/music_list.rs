use ratatui::{
    prelude::*,
    symbols::border,
    widgets::{block::*, *},
};
use std::path::PathBuf;

#[derive(Default)]
pub struct MusicListWidget;

// TODO: Convert this to use a List instead of just paragraph lines
// TODO: Create a queue for music
// TODO: "/" for seaching (perhaps a Search mode on the main App state)
// TODO: Add controls as bottom title
impl MusicListWidget {
    pub fn render(self, area: Rect, buf: &mut Buffer, state: usize, file_list: &Vec<PathBuf>) {
        let title = Title::from(" Files ".bold());
        let controls = Title::from(" Pause <p> | Volume <[> <]> | Speed <(> <)> ");
        let block = Block::default()
            .title(title.alignment(Alignment::Center))
            .title(
                controls
                    .alignment(Alignment::Center)
                    .position(Position::Bottom),
            )
            .borders(Borders::ALL)
            .border_set(border::PLAIN);

        let mut items = vec![];

        for path in file_list {
            items.push(path.to_str().unwrap());
        }

        let list = List::new(items)
            .block(block)
            .highlight_style(Style::default().bold())
            .highlight_symbol(">");
        let mut list_state = ListState::default().with_selected(Some(state));
        StatefulWidget::render(list, area, buf, &mut list_state);

        // Paragraph::new(Text::from(lines.clone()))
        //     .left_aligned()
        //     .scroll((scroll_state, 0))
        //     .wrap(Wrap { trim: false })
        //     .block(block)
        //     .render(area, buf);
        //
        // let scrollbar = Scrollbar::new(ScrollbarOrientation::VerticalRight)
        //     .begin_symbol(Some("↑"))
        //     .end_symbol(Some("↓"));
        //
        // let mut scrollbar_state = ScrollbarState::new(lines.len()).position(scroll_state.into());
        //
        // StatefulWidget::render(
        //     scrollbar,
        //     area.inner(&Margin {
        //         vertical: 1,
        //         horizontal: 0,
        //     }),
        //     buf,
        //     &mut scrollbar_state,
        // );
    }
}
