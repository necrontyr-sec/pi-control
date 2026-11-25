/*
    Author: N3cr0n7yr
    Description: This module build basic command program.
*/
use std::process::Command;
pub fn run_command(action: &str){
    let command = match action {
        "reboot" => Command::new("sudo")
            .arg("/usr/bin/systemctl")
            .arg("reboot")
            .status(),

        "poweroff" => Command::new("sudo")
            .arg("/usr/bin/systemctl")
            .arg("poweroff")
            .status(),

        _ => return,
    };
    match command {
        Ok(status) if !status.success() => {
            println!("Command executed.");
        }

        Ok(status)  => {
            eprintln!("Command failed:{}.", status);
        }

        Err(error) => {
            eprintln!("Execution error: {}", error);
        }

    }

}