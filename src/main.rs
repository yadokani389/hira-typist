use std::io;

use ratatui;

mod app;
mod ui;

fn main() -> io::Result<()> {
    let mut terminal = ratatui::init();
    let mut app = app::App::new();
    let res = app.run(&mut terminal);
    ratatui::restore();
    res
}
