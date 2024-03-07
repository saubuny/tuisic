use std::io;

mod app;
mod info_line;
mod info_panel;
mod music_list;
mod tui;

fn main() -> io::Result<()> {
    let mut terminal = tui::init()?;
    let app_result = app::App::default().run(&mut terminal);
    tui::restore()?;
    app_result
}
