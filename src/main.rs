use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use metadata::media_file::MediaFileMetadata;
use ratatui::{
    prelude::*,
    symbols::border,
    widgets::{block::*, *},
};
use std::io;
use std::process::Command;

mod info_line;
mod music_list;
mod tui;

fn main() -> io::Result<()> {
    let mut terminal = tui::init()?;
    let mut app = App::default();
    let app_result = app.run(&mut terminal);
    tui::restore()?;
    app_result
}

#[derive(Default, Copy, Clone)]
pub struct MusicState {
    progress: u16, // Change to Duration
    paused: bool,
}

#[derive(Default)]
struct App {
    music_list_scroll: u16,
    tab_state: usize,
    music_state: MusicState,
    music_path: String,
    music_metadata: Option<MediaFileMetadata>,
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
            KeyCode::Up => self.scroll_music_list_up(),
            KeyCode::Down => self.scroll_music_list_down(),
            KeyCode::Char('1') => self.tab_state = 0,
            KeyCode::Char('2') => self.tab_state = 1,
            KeyCode::Char('3') => self.tab_state = 2,
            KeyCode::Char('T') => {
                self.music_path = "/home/saubuny/Downloads/vine-boom.mp3".to_string();
                // TODO: Pipe any error messages into a buffer that will be printed on TUI cleanup
                let _ = Command::new("mpv")
                    .arg(&self.music_path)
                    .output()
                    .expect("what");
                self.get_file_metadata();
            }
            _ => {}
        }
    }

    fn get_file_metadata(&mut self) {
        self.music_metadata = MediaFileMetadata::new(&self.music_path).ok();
    }

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
        let main_layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints(vec![Constraint::Percentage(90), Constraint::Percentage(10)])
            .split(area);

        let horizontal_layout = Layout::default()
            .direction(Direction::Horizontal)
            .constraints(vec![Constraint::Percentage(20), Constraint::Percentage(80)])
            .split(main_layout[0]);

        let vertical_layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints(vec![Constraint::Percentage(70), Constraint::Percentage(30)])
            .split(horizontal_layout[1]);

        music_list::MusicListWidget::default().render(
            horizontal_layout[0],
            buf,
            self.music_list_scroll,
        );

        info_line::InfoLineWidget::default().render(main_layout[1], buf, self.music_state);
    }
}
