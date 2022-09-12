use ansi_term::Colour::{Green, Red, Yellow};
use std::env;
use std::fs;
use std::path::Path;
//use std::process::{Child, Command};

pub fn the_user() -> String {
    let user: &str = env!("USER");
    user.to_owned()
}

pub fn read_path() {
    let user: &str = env!("USER");
    let mut _path = format!("/home/{user}/.local/share/fish/.fish_history");
    let read_path = fs::read_to_string(_path);
    println!("{:?}", read_path);
}

#[allow(clippy::useless_format)]
pub fn write_path() {
    let user: &str = env!("USER");
    let mut _path = format!("/home/{user}/.local/share/fish/.fish_history");
    let write_path = fs::write(format!("{_path}"), "");
    println!("{:?}", write_path);
}

pub fn read_file() {
    let whoami = the_user();
    let full_path = format!("/home/{whoami}/.local/share/fish/.fish_history");

    let check_file_read = Path::new(&full_path).exists();

    match check_file_read {
        true => read_path(),
        false => eprintln!("{}", Red.paint("\n.fish_history File Not Found")),
    }
}

pub fn write_to_file() {
    let user: &str = env!("USER");
    let mut _path = format!("/home/{user}/.local/share/fish/.fish_history");

    let check_file_write = Path::new(&_path).exists();

    match check_file_write {
        true => println!(
            "{} {:?} {}",
            Green.paint("\nDeleted Data In .fish_history File."),
            write_path(),
            Yellow.paint("\n\nRESET TERMINAL"),
        ),
        false => eprintln!("{}", Red.paint("\n.fish_history File Not Found")),
    }
}

// pub fn fish_clear_term() -> Child {
//     let fish_clear = Command::new("clear")
//         .args(&["b"])
//         .spawn()
//         .expect("Failed to execute process");
//     fish_clear
// }

// pub fn fish_reset_term() -> Child {
//     let fish_reset = Command::new("reset")
//         .args(&["b"])
//         .spawn()
//         .expect("Failed to execute process");
//     fish_reset
// }

pub fn fish_main() {
    write_to_file();

    //TODO: fix `.fish_history File Not Found` error...
    //figure out how to read/write to a hidden dir (.local)
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
    fn read_history_file() {
        let user: &str = env!("USER");
        assert_eq!(
            format!("/home/{user}/.local/share/fish/.fish_history"),
            "/home/dev/.local/share/fish/.fish_history"
        )
    }

    #[test]
    fn write_to_history_file() {
        let user: &str = env!("USER");
        let mut _path = format!("/home/{user}/.local/share/fish/.fish_history");
        _path = String::from("");
        assert_eq!(_path, String::from(""))
    }

    // #[test]
    // fn clear_terminal_test() {}

    // #[test]
    // fn reset_terminal_test() {}
}
