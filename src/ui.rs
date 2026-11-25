/*
    Author: N3cr0n7yr
    Description: This module build basic command program.
*/

use ratatui::{
    Frame,
    widgets::{
        Block,
        Borders,
        Paragraph,
    },
    layout::{
        Alignment,
        Constraint,
        Direction,
        Layout,
    },
    style::{
        Color,
        Style,
    },
};

pub fn draw(frame: &mut Frame) {

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Min(5),
            Constraint::Length(3),
        ])
        .split(frame.area());

    let title = Paragraph::new(
        "RASPBERRY PI SYSTEM CONTROL"
    )
        .alignment(Alignment::Center)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title(" REBOOT ")
        );

    frame.render_widget(title, chunks[0]);

    let menu = Paragraph::new(
        "1. Reboot Raspberry Pi\n\n\
         2. Shutdown Raspberry Pi\n\n\
         Q. Exit Application"
    )
        .style(
            Style::default().fg(Color::Green)
        )
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title(" Main Menu ")
        );

    frame.render_widget(menu, chunks[1]);

    let footer = Paragraph::new(
        "Select an option using your keyboard"
    )
        .alignment(Alignment::Center);

    frame.render_widget(footer, chunks[2]);
}