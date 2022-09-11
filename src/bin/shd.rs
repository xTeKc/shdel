mod xcli;
use xcli::cli::parse_args;
use xcli::opt::*;

fn main() {
    let _args = match parse_args(Some("shd".to_owned())) {
        Ok(v) => v,
        Err(e) => {
            eprintln!("Error: {}.", e);
            std::process::exit(1);
        }
    };

    //println!("{:#?}", args);

    bash_opt();
    zsh_opt();
    fish_opt();
}
