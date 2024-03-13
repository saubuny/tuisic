use ratatui::{
    prelude::*,
    symbols::border,
    widgets::{block::*, *},
};
use std::path::{Path, PathBuf};

#[derive(Default)]
pub struct MusicListWidget;

impl MusicListWidget {
    pub fn render(
        self,
        area: Rect,
        buf: &mut Buffer,
        state: usize,
        file_list: &Vec<PathBuf>,
        base_path: &Path,
    ) {
        let title = Title::from(" Files ".bold());
        let block = Block::default()
            .title(title.alignment(Alignment::Center))
            .borders(Borders::ALL)
            .border_set(border::PLAIN);

        let mut items: Vec<String> = vec![];
        for path in file_list {
            items.push(
                path.strip_prefix(base_path)
                    .unwrap()
                    .to_str()
                    .unwrap()
                    .to_string(),
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
