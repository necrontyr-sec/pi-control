/*
    Author: N3cr0n7yr
    Description: This module build basic command program.
*/

use std::io;

use crossterm::event::{
    self,
    Event,
    KeyCode,
};

pub fn handle_event() -> io::Result<bool> {
    if let Event::Key(key) = event::read()? {
        match key.code {
            KeyCode::Char('q') | KeyCode::Char('Q') => {
                return Ok(true);
            }

            KeyCode::Char('1') => {
                crate::actions::reboot::execute();
            }

            KeyCode::Char('2') => {
                crate::actions::shutdown::execute();
            }

            _ => {}
        }
    }

    Ok(false)
}