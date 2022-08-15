use std::fs;
use std::env;
use std::path::Path;
use std::process::Command;
// use std::time;
// use std::time::Duration;
use ansi_term::Colour::{Green,};

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
    let owned_user = user.to_owned();
    owned_user
}

pub fn read_file() /*-> std::io::Result<String>*/ {
    let whoami = the_user();
    let full_path = format!("/home/{whoami}/.bash_history");

    let check_file_read = Path::new(&full_path).exists();

    match check_file_read {
        true => read_path(),
        false => eprintln!(".bash_history File Not Found"),
    }

    //Ok(String::from("Read .bash_history File"))
}

pub fn write_to_file() /*-> std::io::Result<String>*/ {
    let user: &str = env!("USER");
    let mut _path = format!("/home/{user}/.bash_history");

    let check_file_write = Path::new(&_path).exists();

    match check_file_write {
        true => write_path(),
        false => eprintln!(".bash_history File Not Found"),
    }

    //Ok(String::from("Wrote to .bash_history File"))
}

pub fn clear_term() -> Command {
    let clear = std::process::Command::new("clear");
    clear
}

pub fn reset_term() -> Command {
    let reset = std::process::Command::new("reset");
    reset
}

pub fn bash_main() {
    let read_file = read_file();
    println!("{:?}", read_file);
    write_to_file();

    // match read_file {
    //     () => println!("{}", Green.paint("\nReset Terminal")),
    //     #[allow(unreachable_patterns)]
    //     _ => (),
    // }
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
        assert_eq!(format!("/home/{user}/.bash_history"), "/home/dev/.bash_history")
    }

    #[test]
    fn write_to_history_file() {
        let user: &str = env!("USER");
        let mut _path = format!("/home/{user}/.bash_history");
        _path = String::from("");
        assert_eq!(_path, String::from(""))
    }

    #[test]
    fn reboot_terminal_test() {

    }

}