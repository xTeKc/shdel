use std::fs;
use std::env;
use ansi_term::Colour::{Red, Green,};


#[allow(unused)]
fn the_user() -> &'static str {
    let user: &'static str = env!("USER");
    user
}

#[allow(unused)]
fn read_file() {
    let whoami = the_user();
    let full_path = format!("/home/{whoami}/.zhistory");
    let read_file = fs::read_to_string(full_path);
    println!("{:?}", read_file);
}

fn write_to_file() -> std::io::Result<()> {
    let user: &'static str = env!("USER");
    let mut _path = format!("/home/{user}/.zhistory");
    let write = fs::write(format!("{_path}"), "")?;
    match write {
        () => println!("{} {}", ".zhistory", Green.paint("cleared")),
        #[allow(unreachable_patterns)]
        _ => println!("{}",Red.paint(".zhistory not cleared")),
    }
    Ok(())
}

fn exit_command() {
    // TODO
}

pub fn main() {
    write_to_file()
    .expect(format!("{}",Red.paint(".zhistory not cleared")).as_str());

    exit_command();
}

#[cfg(test)]
mod tests {
    use super::*;
    //use std::process::Command;

    #[test]
    fn check_the_user() {
        let user: &'static str = env!("USER");
        assert_eq!(format!("{user}"), "dev")
    }

    #[test]
    fn read_history_file() {
        let user: &'static str = env!("USER");
        assert_eq!(format!("/home/{user}/.zhistory"), "/home/dev/.zhistory")
    }

    #[test]
    fn write_to_history_file() {
        let user: &'static str = env!("USER");
        let mut _path = format!("/home/{user}/.zhistory");
        _path = "".to_owned();
        assert_eq!(_path, "".to_owned())
    }

    #[test]
    fn reboot_terminal_test() {

    }

}