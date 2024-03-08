use crate::{
    info_line::InfoLineWidget, info_panel::InfoPanelWidget, music_list::MusicListWidget, tui,
};
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use ratatui::prelude::*;
use std::io;

pub struct MusicState {
    pub progress: i64,
    pub duration: i64,
    pub paused: bool,
    pub volume: i64,
    pub speed: i64,
}

#[derive(Default)]
pub struct App {
    music_list_scroll: u16,
    music_path: String,
    mpv_metadata: String,
    music_state: Option<MusicState>,
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

        if let Some(h) = &mut self.mpv_handler {
            h.observe_property::<i64>("time-pos", 0); // ????
        }

        while !self.exit {
            terminal.draw(|frame| self.render_frame(frame))?;
            self.handle_events()?;
            if let Some(h) = &mut self.mpv_handler {
                match h.wait_event(0.) {
                    Some(mpv::Event::StartFile) => {
                        self.get_file_metadata();
                    }
                    Some(mpv::Event::PropertyChange {
                        name: "time-pos", ..
                    }) => {
                        self.get_music_state();
                    }
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
            // NOTE: Weird bug where if you press this after a the uppercase
            // version of the char before any files are loaded, the program
            // crashes
            KeyCode::Char('t') => {
                if let Some(h) = &mut self.mpv_handler {
                    h.command(&["loadfile", "/home/saubuny/Downloads/woven_web.mp3"]);
                }
            }

            // Definitely a better way to do all of this but it works so oh well
            KeyCode::Char('p') => {
                if let Some(h) = &mut self.mpv_handler {
                    if let Some(m) = &mut self.music_state {
                        if h.get_property::<&str>("pause").unwrap() == "no" {
                            h.set_property("pause", "yes");
                            m.paused = true;
                        } else {
                            h.set_property("pause", "no");
                            m.paused = false;
                        }
                    }
                }
            }
            _ => {}
        }
    }

    fn get_file_metadata(&mut self) {
        if let Some(h) = &mut self.mpv_handler {
            self.mpv_metadata = h.get_property::<&str>("metadata").unwrap().to_owned();
        }
    }

    fn get_music_state(&mut self) {
        if let Some(h) = &mut self.mpv_handler {
            let speed = h.get_property::<i64>("speed").unwrap();
            let duration = h.get_property::<i64>("duration").unwrap();
            let paused = h.get_property::<bool>("pause").unwrap();
            let volume = h.get_property::<i64>("volume").unwrap();
            let progress = h.get_property::<i64>("time-pos").unwrap();

            self.music_state = Some(MusicState {
                speed,
                duration,
                paused,
                volume,
                progress,
            });
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
        InfoLineWidget::default().render(vertical_layout[1], buf, self.music_state.as_ref());
    }
}
