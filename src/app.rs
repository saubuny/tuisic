use crate::{
    info_line::InfoLineWidget, info_panel::InfoPanelWidget, music_list::MusicListWidget, tui,
};
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use mpv::Result;
use ratatui::prelude::*;
use std::io;
use std::time::Duration;

#[derive(Default, Clone, Copy)]
pub struct MusicState {
    pub progress: i64,
    pub duration: i64,
    pub paused: bool,
    pub volume: i64,
    pub speed: f64,
}

#[derive(Default)]
pub struct App {
    music_list_scroll: u16,
    music_path: String,
    mpv_metadata: String,
    music_state: MusicState,
    mpv_handler: Option<mpv::MpvHandler>,
    exit: bool,
}

impl App {
    pub fn run(&mut self, terminal: &mut tui::Tui) -> io::Result<()> {
        let mut mpv_builder = mpv::MpvHandlerBuilder::new().unwrap();
        mpv_builder.set_option("vid", "no").unwrap();
        let mpv_handler = mpv_builder.build().unwrap();
        self.mpv_handler = Some(mpv_handler);

        if let Some(h) = &mut self.mpv_handler {
            let _ = h.observe_property::<i64>("time-pos", 0);
        }

        while !self.exit {
            terminal.draw(|frame| self.render_frame(frame))?;
            self.handle_events()?;
            if let Some(h) = &mut self.mpv_handler {
                match h.wait_event(0.) {
                    Some(mpv::Event::StartFile) => {
                        self.music_state = MusicState::default();
                    }
                    Some(mpv::Event::PropertyChange {
                        name: "time-pos", ..
                    }) => {
                        // Any error is simply from trying to read when it shouldn't, so we can
                        // ignore it
                        if self.get_music_state().is_ok() {
                            let _ = self.get_file_metadata();
                        }
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
        if event::poll(Duration::from_millis(100))? {
            match event::read()? {
                Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
                    self.handle_key_event(key_event)
                }
                _ => {}
            };
        }
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
                    let _ = h.command(&["loadfile", "/home/saubuny/Downloads/tricot.mp3"]);
                }
            }

            // Definitely a better way to do all of this but it works so oh well
            // TODO: Implement speed, volume, and skip queue controls
            KeyCode::Char('p') => {
                if let Some(h) = &mut self.mpv_handler {
                    if h.get_property::<&str>("pause").unwrap() == "no" {
                        let _ = h.set_property("pause", "yes");
                        self.music_state.paused = true;
                    } else {
                        let _ = h.set_property("pause", "no");
                        self.music_state.paused = false;
                    }
                }
            }
            _ => {}
        }
    }

    fn get_file_metadata(&mut self) -> Result<()> {
        if let Some(h) = &mut self.mpv_handler {
            self.mpv_metadata = h
                .get_property::<mpv::OsdString>("filtered-metadata")?
                .string
                .to_string();
        }
        Ok(())
    }

    fn get_music_state(&mut self) -> Result<()> {
        if let Some(h) = &mut self.mpv_handler {
            let speed = h.get_property::<f64>("speed")?;
            let duration = h.get_property::<i64>("duration")?;
            let paused = h.get_property::<bool>("pause")?;
            let volume = h.get_property::<i64>("volume")?;
            let progress = h.get_property::<i64>("playback-time")?;

            self.music_state = MusicState {
                speed,
                duration,
                paused,
                volume,
                progress,
            };
        }
        Ok(())
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
        let vertical_layout =
            Layout::vertical([Constraint::Percentage(100), Constraint::Min(1)]).split(area);
        let horizontal_layout =
            Layout::horizontal([Constraint::Percentage(70), Constraint::Fill(1)])
                .split(vertical_layout[0]);

        MusicListWidget::default().render(horizontal_layout[0], buf, self.music_list_scroll);
        InfoPanelWidget::default().render(horizontal_layout[1], buf, self.mpv_metadata.clone());
        InfoLineWidget::default().render(vertical_layout[1], buf, self.music_state.clone());
    }
}
