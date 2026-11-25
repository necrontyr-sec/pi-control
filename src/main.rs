/*
    Author: N3cr0n7yr
    Description: This module build basic command program.
*/
pub mod app;
pub mod ui;
pub mod event;
pub mod actions;
pub mod system;

use std::io;
use crossterm::{
    execute,
    terminal::{
        LeaveAlternateScreen,
        EnterAlternateScreen,
        disable_raw_mode,
        enable_raw_mode,
    },
};
use ratatui::{
    backend::CrosstermBackend,
    Terminal,
};

fn main() -> io::Result<()> {
    enable_raw_mode()?;

    let mut stdout = io::stdout();

    execute!(stdout, EnterAlternateScreen)?;

    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let response = app::run(&mut terminal);

    disable_raw_mode()?;

    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;

    terminal.show_cursor()?;

    response
}
