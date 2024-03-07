use crate::{
    info_line::InfoLineWidget, info_panel::InfoPanelWidget, music_list::MusicListWidget, tui,
};
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use ratatui::prelude::*;
use std::io;

#[derive(Default)]
pub struct App {
    music_list_scroll: u16,
    music_path: String,
    mpv_output: String,
    exit: bool,
}

impl App {
    pub fn run(&mut self, terminal: &mut tui::Tui) -> io::Result<()> {
        while !self.exit {
            terminal.draw(|frame| self.render_frame(frame))?;
            self.handle_events()?;
        }
        Ok(())
    }

    fn render_frame(&self, frame: &mut Frame) {
        frame.render_widget(self, frame.size());
    }

    fn handle_events(&mut self) -> io::Result<()> {
        match event::read()? {
            Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
                self.handle_key_event(key_event)
            }
            _ => {}
        };
        Ok(())
    }

    fn handle_key_event(&mut self, key_event: KeyEvent) {
        match key_event.code {
            KeyCode::Char('q') => self.exit(),
            KeyCode::Up | KeyCode::Char('k') => self.scroll_music_list_up(),
            KeyCode::Down | KeyCode::Char('j') => self.scroll_music_list_down(),
            // Test for running audio, remove this later when selection is implemented
            KeyCode::Char('T') => {}
            _ => {}
        }
    }

    // Parse MPV output
    fn get_file_metadata(&mut self) {}

    fn scroll_music_list_up(&mut self) {
        self.music_list_scroll = self.music_list_scroll.saturating_sub(1);
    }

    fn scroll_music_list_down(&mut self) {
        self.music_list_scroll = self.music_list_scroll.saturating_add(1);
    }

    fn exit(&mut self) {
        self.exit = true;
    }
}

impl Widget for &App {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let vertical_layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints(vec![Constraint::Percentage(90), Constraint::Percentage(10)])
            .split(area);

        let horizontal_layout = Layout::default()
            .direction(Direction::Horizontal)
            .constraints(vec![Constraint::Percentage(70), Constraint::Percentage(30)])
            .split(vertical_layout[0]);

        MusicListWidget::default().render(horizontal_layout[0], buf, self.music_list_scroll);

        InfoPanelWidget::default().render(horizontal_layout[1], buf);
        InfoLineWidget::default().render(vertical_layout[1], buf);
    }
}
