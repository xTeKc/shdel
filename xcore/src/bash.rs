use std::fs;
use std::env;
#[allow(unused)]
use std::path::Path;
use ansi_term::Colour::{Red, Green,};


#[allow(unused)]
pub fn the_user() -> String {
    let user: &str = env!("USER");
    let owned_user = user.to_owned();
    owned_user
}

#[allow(unused)]
pub fn read_file() {
    let whoami = the_user();
    let full_path = format!("/home/{whoami}/.bash_history");
    let read_file = fs::read_to_string(full_path);
    println!("{:?}", read_file);
}

pub fn write_to_file() -> std::io::Result<()> {
    let user: &str = env!("USER");
    let mut _path = format!("/home/{user}/.bash_history");
    let write = fs::write(format!("{_path}"), "")?;
    match write {
        () => println!("{} {}", ".bash_history", Green.paint("cleared")),
        #[allow(unreachable_patterns)]
        _ => println!("{}",Red.paint(".bash_history not cleared")),
    }
    Ok(())
}

pub fn exit_command() {
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