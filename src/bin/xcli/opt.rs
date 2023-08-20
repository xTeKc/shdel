/******************************
 *  Copyright (c) xTekC.      *
 *  Licensed under MPL-2.0.   *
 *  See LICENSE for details.  *
 *                            *
 ******************************/

use pico_args::Arguments;
use shdel::xcore::bash::bash_main;
use shdel::xcore::fish::fish_main;
use shdel::xcore::zsh::zsh_main;

pub fn bash_opt() {
    let mut bash_arg = Arguments::from_env();

    match bash_arg.contains(["-b", "--bash"]) {
        true => bash_main(),
        false => (),
    }
}

pub fn zsh_opt() {
    let mut zsh_arg = Arguments::from_env();

    match zsh_arg.contains(["-z", "--zsh"]) {
        true => zsh_main(),
        false => (),
    }
}

pub fn fish_opt() {
    let mut fish_arg = Arguments::from_env();

    match fish_arg.contains(["-f", "--fish"]) {
        true => fish_main(),
        false => (),
    }
}
