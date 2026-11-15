// The rei package manager
// A zero bloat package manager designed to work on all linux distros
// Default package manager of Nixel-Linux
// Part of Buchemium Coreutils project

mod cli;
mod commands;
mod error;
mod manifest;

use cli::process_subcommand;
use cli::NextCommand;
use std::env;
use std::process::ExitCode;

fn main() -> ExitCode {
    if run().is_err() {
        return ExitCode::from(1);
    }

    return ExitCode::from(0);
}

fn run() -> Result<(), error::Error> {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage: rei <subcommand> <args_of_subcommand>");
        return Err(error::Error::InvalidArguments); // when run it prints Error: InvalidArguments, how fix?
    }

    init_rei_subsystem(process_subcommand(&args));

    return Ok(());
}

fn init_rei_subsystem(next_command: Result<cli::NextCommand, error::Error>) {
    match next_command {
        Err(error::Error::InvalidSubcommand) => eprintln!("rei: Invalid subcommand"),
        Err(error::Error::InvalidArguments) => eprintln!("rei: Invalid arguments"),
        Ok(something) => match something {
            NextCommand::FetchAndBuild { package } => commands::get::get(package),
            NextCommand::FetchAndBinaryOnly { package } => {}
            NextCommand::RemovePackage { package } => {}
            NextCommand::SyncIndex => {}
            NextCommand::Upgrade {
                package: Some(package),
            } => {}
            NextCommand::Upgrade { package: None } => {}
            NextCommand::ListPackages => {}
            NextCommand::InfoPackage { package } => {}
            NextCommand::CheckManifest { manifest } => {}
            NextCommand::NewPackageProject { name: Some(name) } => {}
            NextCommand::NewPackageProject { name: None } => {}
        },
    }
}
