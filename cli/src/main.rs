#[allow(unused)]
use xcore::{bash, zsh};
use clap::*;

fn get_args_builder() {
    let _matches = App::new("\nshelldel")
        .version("[ 0.1.0 ]")
        .about("Delete data from shell history file(s)")
        //.author("xTeKc")
        .arg(
            Arg::new("bash")
            .short('b')
            .long("bash")
            .help("Delete data from bash history file")
        )
        .arg(
            Arg::new("zsh")
            .short('z')
            .long("zsh")
            .help("Delete data from zsh history file")
        )
        .get_matches();

        let _bash_h = bash::the_user();
        let _zsh_h = zsh::the_user();

    if let Some(_bash_h) = _matches.value_of("bash") {
        println!("{}", _bash_h);
    }

    if let Some(_zsh_h) = _matches.value_of("zsh") {
        println!("{}", _zsh_h);
    }
}

    // #[derive(Parser)]
    // #[clap(author, version, about, long_about = None)]
    // pub struct TheCli {
    //     /// Delete data from bash history file
    //     #[clap(value_parser)]
    //     bash: String,
    //     /// Delete data from zsh history file
    //     #[clap(value_parser)]
    //     zsh: String
    // }


fn main() {
    get_args_builder();
}