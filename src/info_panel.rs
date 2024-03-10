use ratatui::{
    prelude::*,
    symbols::border,
    widgets::{block::*, *},
};

#[derive(Default)]
pub struct InfoPanelWidget;

impl InfoPanelWidget {
    pub fn render(self, area: Rect, buf: &mut Buffer, metadata: String) {
        let layout = Layout::vertical([Constraint::Fill(1), Constraint::Fill(1)]).split(area);
        let title = Title::from(" Metadata ".bold());
        let block = Block::default()
            .title(title.alignment(Alignment::Center))
            .borders(Borders::ALL)
            .border_set(border::PLAIN);
        let mut lines = vec![];

        let mut str = String::from("");
        for (_, ch) in metadata.chars().enumerate() {
            str.push(ch);
            if ch == '\n' {
                lines.push(Line::from(str));
                str = String::from("");
            }
        }

        Paragraph::new(lines).block(block).render(layout[0], buf);
        let lines = vec![
            Line::from(format!("{:7}{}", "Pause", "<p>")),
            Line::from(format!("{:7}{}", "Volume", "<[> <]>")),
            Line::from(format!("{:7}{}", "Speed", "<(> <)>")),
            Line::from(format!("{:7}{}", "Play", "<Enter>")),
        ];

        let title = Title::from(" Controls ".bold());
        let block = Block::default()
            .title(title.alignment(Alignment::Center))
            .borders(Borders::ALL)
            .border_set(border::PLAIN);
        Paragraph::new(lines).block(block).render(layout[1], buf);
    }
}
