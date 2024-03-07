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
    mpv_metadata: String,
    mpv_handler: Option<mpv::MpvHandler>,
    exit: bool,
}

impl App {
    pub fn run(&mut self, terminal: &mut tui::Tui) -> io::Result<()> {
        let mut mpv_builder = mpv::MpvHandlerBuilder::new().expect("Failed to create mpv builder.");
        mpv_builder
            .set_option("vid", "no")
            .expect("Failed to turn off video player option.");
        let mpv_handler = mpv_builder.build().expect("Failed to build mpv handle.");
        self.mpv_handler = Some(mpv_handler);
        while !self.exit {
            terminal.draw(|frame| self.render_frame(frame))?;
            self.handle_events()?;
            if let Some(h) = &mut self.mpv_handler {
                match h.wait_event(0.) {
                    Some(mpv::Event::StartFile) => self.get_file_metadata(),
                    _ => {}
                }
            }
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
            KeyCode::Char('t') => {
                if let Some(h) = &mut self.mpv_handler {
                    h.command(&["loadfile", "/home/saubuny/Downloads/woven_web.mp3"]);
                }
            }
            KeyCode::Char('p') => {
                // I feel like there's a more elegant way to do this but oh well as mpv::MpvFormat
                if let Some(h) = &mut self.mpv_handler {
                    if h.get_property::<&str>("pause").unwrap() == "no" {
                        h.set_property("pause", "yes");
                    } else {
                        h.set_property("pause", "no");
                    }
                }
            }
            _ => {}
        }
    }

    // Parse MPV output
    fn get_file_metadata(&mut self) {
        if let Some(h) = &mut self.mpv_handler {
            self.mpv_metadata = h.get_property::<&str>("metadata").unwrap().to_owned();
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
            .constraints(vec![Constraint::Percentage(90), Constraint::Percentage(10)])
            .split(area);

        let horizontal_layout = Layout::default()
            .direction(Direction::Horizontal)
            .constraints(vec![Constraint::Percentage(70), Constraint::Percentage(30)])
            .split(vertical_layout[0]);

        MusicListWidget::default().render(horizontal_layout[0], buf, self.music_list_scroll);

        InfoPanelWidget::default().render(horizontal_layout[1], buf, self.mpv_metadata.clone());
        InfoLineWidget::default().render(vertical_layout[1], buf);
    }
}
