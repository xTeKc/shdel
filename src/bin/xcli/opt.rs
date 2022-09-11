use shelldel::shells::bash::bash_main;
use shelldel::shells::zsh::zsh_main;
use shelldel::shells::fish::fish_main;
use pico_args::Arguments;

pub fn bash_opt() {
    let mut bash_arg = Arguments::from_env();

    if bash_arg.contains(["-b", "--bash"]) {
        bash_main()
    }
}

pub fn zsh_opt() {
    let mut zsh_arg = Arguments::from_env();

    if zsh_arg.contains(["-z", "--zsh"]) {
        zsh_main()
    }
}

pub fn fish_opt() {
    let mut fish_arg = Arguments::from_env();

    if fish_arg.contains(["-f", "--fish"]) {
        fish_main()
    }
}