use ratatui::{
    prelude::*,
    symbols::border,
    widgets::{block::*, *},
};
use std::path::PathBuf;

#[derive(Default)]
pub struct MusicListWidget;

impl MusicListWidget {
    pub fn render(
        self,
        area: Rect,
        buf: &mut Buffer,
        state: usize,
        file_list: &Vec<PathBuf>,
        base_path: &PathBuf,
    ) {
        let title = Title::from(" Files ".bold());
        let block = Block::default()
            .title(title.alignment(Alignment::Center))
            .borders(Borders::ALL)
            .border_set(border::PLAIN);

        let mut items: Vec<String> = vec![];
        for path in file_list {
            self.add_path(&mut items, path, base_path);
        }

        let list = List::new(items)
            .block(block)
            .highlight_style(Style::default().bold())
            .highlight_symbol("> ");
        let mut list_state = ListState::default().with_selected(Some(state));
        StatefulWidget::render(list, area, buf, &mut list_state);
    }

    fn add_path(&self, items: &mut Vec<String>, path: &PathBuf, base_path: &PathBuf) {
        items.push(
            path.strip_prefix(base_path.clone())
                .unwrap()
                .to_str()
                .unwrap()
                .to_string(),
        );
    }
}
