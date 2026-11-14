mod cli;
mod commands;
mod error;
mod manifest;

use cli::process_subcommand;
use cli::NextCommand;
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

fn init_rei_subsystem(next_command: Result<cli::NextCommand, error::Error>) {
    match next_command {
        Err(error::Error::InvalidSubcommand) => eprintln!("Invalid subcommand"),
        Err(error::Error::InvalidArguments) => eprintln!("Invalid arguments"),
        Ok(something) => match something {
            NextCommand::FetchAndBuild { package } => println!("Works"),
            _ => todo!(),
        },
    }
}
