/// The rei package manager
/// A zero bloat package manager designed to work on all linux distros
/// Default package manager of Nixel-Linux
/// Part of Buchemium Coreutils project
/// Created by Azure-6-lgtm (Ujan Barman)
mod cli;
mod commands;
mod error;
mod manifest;

use cli::process_subcommand;
use cli::NextCommand;
use std::env;
use std::process::ExitCode;

fn main() -> ExitCode {
    if let Err(e) = run() {
        eprintln!("{}", e);
        return ExitCode::from(1);
    }

    ExitCode::from(0)
}

fn run() -> Result<(), error::Error> {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage: rei <subcommand> <args_of_subcommand>");
        return Err(error::Error::InvalidArguments);
    }

    let next_command = process_subcommand(&args)?;
    init_rei_subsystem(next_command)
}

fn init_rei_subsystem(next_command: cli::NextCommand) -> Result<(), error::Error> {
    match next_command {
        NextCommand::FetchAndBuild { package } => commands::get::get_package(package),
        NextCommand::FetchAndBinaryOnly { package } => commands::bin::get_binary(package),
        NextCommand::RemovePackage { package } => commands::remove::remove_package(package),
        NextCommand::SyncIndex => commands::sync::sync_index(),
        NextCommand::Upgrade {
            package: Some(package),
        } => commands::upgrade::upgrade_package(package),
        NextCommand::Upgrade { package: None } => commands::upgrade::upgrade_package(package),
        NextCommand::ListPackages => commands::list::list_packages(),
        NextCommand::InfoPackage { package } => commands::info::info_package(package),
        NextCommand::CheckManifest { manifest } => commands::check::check_manifest(manifest),
        NextCommand::NewPackageProject { name: Some(name) } => commands::new::new_project(name),
        NextCommand::NewPackageProject { name: None } => commands::new::new_project(name),
    }
}
