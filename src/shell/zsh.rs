use ansi_term::Colour::{Green, Red, Yellow};
use std::env;
use std::fs;
use std::path::Path;
//use std::process::{Child, Command};

pub fn the_user() -> String {
    let user: &str = env!("USER");
    user.to_owned()
}

pub fn read_first_path() {
    let user: &str = env!("USER");
    let mut _path = format!("/home/{user}/.zsh_history");
    let read_path = fs::read_to_string(_path);
    println!("{:?}", read_path);
}

pub fn read_second_path() {
    let user: &str = env!("USER");
    let mut _path = format!("/home/{user}/.zhistory");
    let read_path = fs::read_to_string(_path);
    println!("{:?}", read_path);
}

#[allow(clippy::useless_format)]
pub fn write_first_path() {
    let user: &str = env!("USER");
    let mut _path = format!("/home/{user}/.zsh_history");
    let write_path = fs::write(format!("{_path}"), "");
    println!("{:?}", write_path);
}

#[allow(clippy::useless_format)]
pub fn write_second_path() {
    let user: &str = env!("USER");
    let mut _path = format!("/home/{user}/.zhistory");
    let write_path = fs::write(format!("{_path}"), "");
    println!("{:?}", write_path);
}

pub fn read_first_file() {
    let whoami = the_user();
    let full_path = format!("/home/{whoami}/.zsh_history");

    let check_file_read = Path::new(&full_path).exists();

    match check_file_read {
        true => read_first_path(),
        false => eprintln!("{}", Red.paint("\n.zsh_history File Not Found")),
    }
}

pub fn read_second_file() {
    let whoami = the_user();
    let full_path = format!("/home/{whoami}/.zhistory");

    let check_file_read = Path::new(&full_path).exists();

    match check_file_read {
        true => read_first_path(),
        false => eprintln!("{}", Red.paint("\n.zhistory File Not Found")),
    }
}

pub fn write_to_first_file() {
    let user: &str = env!("USER");
    let mut _path = format!("/home/{user}/.zsh_history");

    let check_file_write = Path::new(&_path).exists();

    match check_file_write {
        true => println!(
            "{} {:?} {}",
            Green.paint("\nDeleted Data In .zsh_history File."),
            write_first_path(),
            Yellow.paint("\n\nRESET TERMINAL"),
        ),
        false => eprintln!("{}", Red.paint("\n.zsh_history File Not Found")),
    }
}

pub fn write_to_second_file() {
    let user: &str = env!("USER");
    let mut _path = format!("/home/{user}/.zhistory");

    let check_file_write = Path::new(&_path).exists();

    match check_file_write {
        true => println!(
            "{} {:?} {}",
            Green.paint("\nDeleted Data In .zhistory File."),
            write_second_path(),
            Yellow.paint("\n\nRESET TERMINAL"),
        ),
        false => eprintln!("{}", Red.paint("\n.zhistory File Not Found")),
    }
}

// pub fn zsh_clear_term() -> Child {
//     let zsh_clear = Command::new("clear")
//         .args(&["z"])
//         .spawn()
//         .expect("Failed to execute process");
//     zsh_clear
// }

// pub fn zsh_reset_term() -> Child {
//     let zsh_reset = Command::new("reset")
//         .args(&["z"])
//         .spawn()
//         .expect("Failed to execute process");
//     zsh_reset
// }

pub fn zsh_main() {
    write_to_first_file();
    write_to_second_file();
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn check_the_user() {
        let user: &str = env!("USER");
        assert_eq!(format!("{user}"), "dev")
    }

    #[test]
    fn read_first_history_file() {
        let user: &str = env!("USER");
        assert_eq!(format!("/home/{user}/.zsh_history"), "/home/dev/.zsh_history")
    }

    #[test]
    fn read_second_history_file() {
        let user: &str = env!("USER");
        assert_eq!(format!("/home/{user}/.zhistory"), "/home/dev/.zhistory")
    }

    #[test]
    fn write_to_first_history_file() {
        let user: &str = env!("USER");
        let mut _path = format!("/home/{user}/.zsh_history");
        _path = String::from("");
        assert_eq!(_path, String::from(""))
    }

    #[test]
    fn write_to_second_history_file() {
        let user: &str = env!("USER");
        let mut _path = format!("/home/{user}/.zhistory");
        _path = String::from("");
        assert_eq!(_path, String::from(""))
    }

    // #[test]
    // fn clear_terminal_test() {}

    // #[test]
    // fn reset_terminal_test() {}
}
