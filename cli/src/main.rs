#[allow(unused)]
use xcore::{bash, zsh};
use clap::{Parser, Subcommand};

#[derive(Debug, Parser)]
#[clap(about, version, author,)]
struct TheShell {
    /// Delete data from bash
    #[clap(short = 'b', long)]
    bash: String,
    /// Delete data from zsh
    #[clap(short = 'z', long)]
    zsh: String,
}

#[derive(Parser)]
#[clap(about, version, author)]
struct Output {
    #[clap(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Bash (TheShell),
    Zsh (TheShell),
}

fn main() {
    let the_shell = TheShell {
        bash: bash::the_user(),
        zsh: zsh::the_user(),
    };

    //let _parse_shell = TheShell::parse();

    println!("{:#?}", the_shell);

    
}