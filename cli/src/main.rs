use clap::{App, Arg};
use xcore::{bash, zsh};

fn main() {
    let matches = App::new("shelldel")
        .about("Delete data from shell history file(s)")
        .version("0.1.0")
        .arg(
            Arg::new("bash")
                .short('b')
                .long("bash")
                .help("Delete bash data"),
        )
        .arg(
            Arg::new("zsh")
                .short('z')
                .long("zsh")
                .help("Delete zsh data"),
        )
        .get_matches();

    match matches.contains_id("bash") {
        true => bash::bash_main(),
        false => (),
    };

    match matches.contains_id("zsh") {
        true => zsh::zsh_main(),
        false => (),
    };
}
