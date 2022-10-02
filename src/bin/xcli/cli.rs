use crate::xcli::util::*;
use pico_args::Arguments;

#[derive(Debug)]
pub struct Args {
    pub bash: bool,
    pub zsh: bool,
    pub fish: bool,
}

pub fn parse_args(bin_name: Option<String>) -> Result<Args, pico_args::Error> {
    let mut pargs = Arguments::from_env();

    if pargs.contains(["-h", "--help"]) {
        let help = format!(
            r#"
{GREEN}{name}{RESET} {version}
{desc}

{YELLOW}USAGE:{RESET}
    {name} [OPTION]

{YELLOW}OPTIONS:{RESET}
    {GREEN}-b{RESET}, {GREEN}--bash{RESET}                      Delete bash data
    {GREEN}-z{RESET}, {GREEN}--zsh{RESET}                       Delete zsh data
    {GREEN}-f{RESET}, {GREEN}--fish{RESET}                      Delete fish data
    {GREEN}-h{RESET}, {GREEN}--help{RESET}                      Print help information
    {GREEN}-v{RESET}, {GREEN}--version{RESET}                   Print version information
    "#,
            name = bin_name.unwrap_or_else(|| env!("CARGO_PKG_NAME").to_owned()),
            version = env!("CARGO_PKG_VERSION"),
            //authors = env!("CARGO_PKG_AUTHORS"),
            desc = env!("CARGO_PKG_DESCRIPTION"),
        );

        println!("{}", help);
        std::process::exit(0);
    }

    if pargs.contains(["-v", "--version"]) {
        let version = format!(
            r#"
            {GREEN}{name}{RESET} {version}
        "#,
            name = bin_name.unwrap_or_else(|| env!("CARGO_PKG_NAME").to_owned()),
            version = env!("CARGO_PKG_VERSION"),
        );

        println!("{}", version);
        std::process::exit(0);
    }

    let args = Args {
        bash: pargs.contains(["-b", "--bash"]),
        zsh: pargs.contains(["-z", "--zsh"]),
        fish: pargs.contains(["-f", "--fish"]),
    };

    Ok(args)
}
