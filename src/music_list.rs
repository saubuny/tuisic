use ratatui::{
    prelude::*,
    symbols::border,
    widgets::{block::*, *},
};
use std::path::PathBuf;

#[derive(Default)]
pub struct MusicListWidget;

// TODO: "/" for seaching (perhaps a Search mode on the main App state)
// A search mode would show a fzf-like interface with BottomToTop directed list
impl MusicListWidget {
    pub fn render(
        self,
        area: Rect,
        buf: &mut Buffer,
        state: usize,
        file_list: &Vec<PathBuf>,
        base_path: PathBuf,
    ) {
        let title = Title::from(" Files ".bold());
        let block = Block::default()
            .title(title.alignment(Alignment::Center))
            .borders(Borders::ALL)
            .border_set(border::PLAIN);

        let mut items = vec![];
        for path in file_list {
            items.push(
                path.strip_prefix(base_path.clone())
                    .unwrap()
                    .to_str()
                    .unwrap(),
            );
        }

        let list = List::new(items)
            .block(block)
            .highlight_style(Style::default().bold())
            .highlight_symbol("> ");
        let mut list_state = ListState::default().with_selected(Some(state));
        StatefulWidget::render(list, area, buf, &mut list_state);
    }
}
