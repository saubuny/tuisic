use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use ratatui::{
    prelude::*,
    symbols::border,
    widgets::{block::*, *},
};
use std::io;
use std::process::Command;

mod music_list;
mod tui;

fn main() -> io::Result<()> {
    let mut terminal = tui::init()?;
    let mut app = App::default();
    let app_result = app.run(&mut terminal);
    tui::restore()?;
    app_result
}

#[derive(Default)]
struct App {
    music_list_scroll: u16,
    tab_state: usize,
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
            _ => {}
        }
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
        let vertical_layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints(vec![Constraint::Percentage(70), Constraint::Percentage(30)])
            .split(area);

        let horizontal_layout = Layout::default()
            .direction(Direction::Horizontal)
            .constraints(vec![Constraint::Percentage(50), Constraint::Percentage(50)])
            .split(vertical_layout[1]);

        // TODO: Add sparkline widget for more info
        music_list::MusicListWidget::default().render(
            horizontal_layout[0],
            buf,
            self.music_list_scroll,
        );
    }
}
