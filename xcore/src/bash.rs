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
    let full_path = format!("/home/{whoami}/.bash_history");
    let read_file = fs::read_to_string(full_path);
    println!("{:?}", read_file);
}

fn write_to_file() -> std::io::Result<()> {
    let user: &'static str = env!("USER");
    let mut _path = format!("/home/{user}/.bash_history");
    let write = fs::write(format!("{_path}"), "")?;
    match write {
        () => println!("{} {}", ".bash_history", Green.paint("cleared")),
        #[allow(unreachable_patterns)]
        _ => println!("{}",Red.paint(".bash_history not cleared")),
    }
    Ok(())
}

fn exit_command() {
    // TODO
}

pub fn main() {
    write_to_file()
    .expect(format!("{}",Red.paint(".bash_history not cleared")).as_str());

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
        assert_eq!(format!("/home/{user}/.bash_history"), "/home/dev/.bash_history")
    }

    #[test]
    fn write_to_history_file() {
        let user: &'static str = env!("USER");
        let mut _path = format!("/home/{user}/.bash_history");
        _path = None;
        assert_eq!(_path, None)
    }

    #[test]
    fn reboot_terminal_test() {

    }

}