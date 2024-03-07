use ratatui::{
    prelude::*,
    symbols::border,
    widgets::{block::*, *},
};
use serde_json::Value;

#[derive(Default)]
pub struct InfoPanelWidget;

impl InfoPanelWidget {
    pub fn render(self, area: Rect, buf: &mut Buffer, metadata: String) {
        let title = Title::from("Metadata".bold());
        let block = Block::default()
            .borders(Borders::ALL)
            .title(title)
            .border_set(border::PLAIN);
        let mut lines = vec![];

        let v: Option<Value> = serde_json::from_str(&metadata).ok();
        if let Some(v) = v {
            lines = vec![
                Line::from(vec![
                    Span::raw("Title: "),
                    Span::raw(v["title"].as_str().unwrap().to_owned()),
                ]),
                Line::from(vec![
                    Span::raw("Artist: "),
                    Span::raw(v["artist"].as_str().unwrap().to_owned()),
                ]),
                Line::from(vec![
                    Span::raw("Album: "),
                    Span::raw(v["album"].as_str().unwrap().to_owned()),
                ]),
                Line::from(vec![
                    Span::raw("Date: "),
                    Span::raw(v["date"].as_str().unwrap().to_owned()),
                ]),
            ];
        }

        Paragraph::new(lines).block(block).render(area, buf);
    }
}
