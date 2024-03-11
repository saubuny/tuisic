use crate::{
    info_line::InfoLineWidget, info_panel::InfoPanelWidget, music_list::MusicListWidget, tui,
};
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use mpv::Result;
use ratatui::prelude::*;
use std::{
    collections::VecDeque,
    fs, io,
    ops::{Add, Sub},
    path::PathBuf,
    time::Duration,
};

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
    list_state: usize,
    file_list: Vec<PathBuf>,
    base_path: PathBuf,
    queue: VecDeque<PathBuf>,
    is_playing: bool,
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
            let _ = h.observe_property::<i64>("playback-time", 0);
        }

        let music_dir = xdg_user::music().expect("Please set up your XDG user directories.");
        if let Some(d) = music_dir {
            self.base_path = d.clone();
            for path in fs::read_dir(d)? {
                let path = path.unwrap().path();
                if path.is_dir() {
                    for path in fs::read_dir(path)? {
                        let path = path.unwrap().path();
                        self.file_list.push(path);
                    }
                } else {
                    self.file_list.push(path);
                }
            }
        }

        while !self.exit {
            terminal.draw(|frame| self.render_frame(frame))?;
            self.handle_events()?;
            if let Some(h) = &mut self.mpv_handler {
                match h.wait_event(0.) {
                    Some(mpv::Event::StartFile) => {
                        self.music_state = MusicState::default();
                        self.is_playing = true;
                    }
                    Some(mpv::Event::EndFile(..)) => {
                        if self.is_playing {
                            self.skip_queue();
                        }
                    }
                    Some(mpv::Event::PropertyChange {
                        name: "playback-time",
                        ..
                    }) => {
                        // We have to delay fetching the metadata because mpv likes to make everything
                        // crash if you do it too early
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
            KeyCode::Esc => self.exit(),
            KeyCode::Up | KeyCode::Char('k') => self.scroll_music_list_up(),
            KeyCode::Down | KeyCode::Char('j') => self.scroll_music_list_down(),
            KeyCode::Enter => {
                if let Some(h) = &mut self.mpv_handler {
                    let _ = h.command(&[
                        "loadfile",
                        self.file_list[self.list_state].to_str().unwrap(),
                    ]);
                }
            }

            KeyCode::Char('q') => self.add_to_queue(),
            KeyCode::Char('s') => self.skip_queue(),

            // Definitely a better way to do all of this but it works so oh well
            KeyCode::Char('p') => {
                if let Some(h) = &mut self.mpv_handler {
                    if h.get_property::<&str>("pause").unwrap() == "no" {
                        let _ = h.set_property("pause", "yes");
                    } else {
                        let _ = h.set_property("pause", "no");
                    }
                    let _ = self.get_music_state();
                }
            }

            KeyCode::Char('(') => {
                if let Some(h) = &mut self.mpv_handler {
                    let _ =
                        h.set_property("speed", self.music_state.speed.sub(0.1).clamp(0.5, 2.0));
                    let _ = self.get_music_state();
                }
            }

            KeyCode::Char(')') => {
                if let Some(h) = &mut self.mpv_handler {
                    let _ =
                        h.set_property("speed", self.music_state.speed.add(0.1).clamp(0.5, 2.0));
                    let _ = self.get_music_state();
                }
            }

            KeyCode::Char('[') => {
                if let Some(h) = &mut self.mpv_handler {
                    let _ = h.set_property("volume", self.music_state.volume.sub(5).clamp(0, 130));
                    let _ = self.get_music_state();
                }
            }

            KeyCode::Char(']') => {
                if let Some(h) = &mut self.mpv_handler {
                    let _ = h.set_property("volume", self.music_state.volume.add(5).clamp(0, 130));
                    let _ = self.get_music_state();
                }
            }
            _ => {}
        }
    }

    fn add_to_queue(&mut self) {
        if let Some(h) = &mut self.mpv_handler {
            let file = self.file_list[self.list_state].clone();
            if self.is_playing {
                self.queue.push_back(file);
            } else {
                let _ = h.command(&["loadfile", file.to_str().unwrap()]);
            }
        }
    }

    fn skip_queue(&mut self) {
        self.is_playing = false;
        if let Some(h) = &mut self.mpv_handler {
            let file = self.queue.pop_front().unwrap_or("".into());
            let file = file.to_str().unwrap();
            if !file.is_empty() {
                let _ = h.command(&["loadfile", file]);
            }
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
        self.list_state = self.list_state.saturating_sub(1);
    }

    fn scroll_music_list_down(&mut self) {
        self.list_state = self.list_state.add(1).clamp(0, self.file_list.len() - 1);
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

        MusicListWidget::default().render(
            horizontal_layout[0],
            buf,
            self.list_state,
            &self.file_list,
            &self.base_path,
        );

        InfoPanelWidget::default().render(horizontal_layout[1], buf, self.mpv_metadata.clone());
        InfoLineWidget::default().render(vertical_layout[1], buf, self.music_state.clone());
    }
}
