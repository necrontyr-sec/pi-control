/*
    Author: N3cr0n7yr
    Description: This module build basic command program.
*/

use std::io;

use ratatui::{
    backend::Backend,
    Terminal,
};

pub fn run<B: Backend>(terminal: &mut Terminal<B>,) -> io::Result<()> {
    loop {
        terminal.draw(|frame| {
            crate::ui::draw(frame);
        })?;

        if crate::event::handle_event()? {
            break;
        }
    }

    Ok(())
}