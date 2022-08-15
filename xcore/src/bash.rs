use std::env;
use std::fs;
use std::path::Path;
use std::process::Command;
// use std::time;
// use std::time::Duration;
use ansi_term::Colour::{Green, Red, Yellow};

pub fn read_path() {
    let user: &str = env!("USER");
    let mut _path = format!("/home/{user}/.bash_history");
    let read_path = fs::read_to_string(_path);
    println!("{:?}", read_path);
}

pub fn write_path() {
    let user: &str = env!("USER");
    let mut _path = format!("/home/{user}/.bash_history");
    let write_path = fs::write(format!("{_path}"), "");
    println!("{:?}", write_path);
}

pub fn the_user() -> String {
    let user: &str = env!("USER");
    user.to_owned()
}

pub fn read_file() {
    let whoami = the_user();
    let full_path = format!("/home/{whoami}/.bash_history");

    let check_file_read = Path::new(&full_path).exists();

    match check_file_read {
        true => read_path(),
        false => eprintln!("{}", Red.paint("\n.bash_history File Not Found")),
    }
}

pub fn write_to_file() {
    let user: &str = env!("USER");
    let mut _path = format!("/home/{user}/.bash_history");

    let check_file_write = Path::new(&_path).exists();

    match check_file_write {
        true => println!(
            "{} {:?} {}",
            Green.paint("\nDeleted Data In .bash_history File."),
            write_path(),
            Yellow.paint("\n\nRESET TERMINAL"),
        ),
        false => eprintln!("{}", Red.paint("\n.bash_history File Not Found")),
    }
}

pub fn clear_term() -> Command {
    std::process::Command::new("clear")
}

pub fn reset_term() -> Command {
    std::process::Command::new("reset")
}

pub fn bash_main() {
    write_to_file();
}

#[cfg(test)]
mod tests {
    use super::*;
    //use std::process::Command;

    #[test]
    fn check_the_user() {
        let user: &str = env!("USER");
        assert_eq!(format!("{user}"), "dev")
    }

    #[test]
    fn read_history_file() {
        let user: &str = env!("USER");
        assert_eq!(format!("/home/{user}/.bash_history"),
        "/home/dev/.bash_history")
    }

    #[test]
    fn write_to_history_file() {
        let user: &str = env!("USER");
        let mut _path = format!("/home/{user}/.bash_history");
        _path = String::from("");
        assert_eq!(_path, String::from(""))
    }

    #[test]
    fn clear_terminal_test() {}

    #[test]
    fn reset_terminal_test() {}
}
