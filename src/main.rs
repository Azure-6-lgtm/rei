mod cli;
mod commands;
mod error;
mod manifest;

use cli::process_subcommand;
use std::env;

fn main() -> Result<(), std::io::Error> {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage: rei <subcommand> <args_of_subcommand>");
        eprintln!("Examole: rei get firefox");
        return Ok(());
    }

    init_rei_subsystem(process_subcommand(&args));

    return Ok(());
}

fn init_rei_subsystem(next_command: Result<cli::NextCommand, error::Error>) {}
