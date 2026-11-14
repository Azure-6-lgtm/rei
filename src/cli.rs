use crate::error::Error;

pub enum NextCommand {
    FetchAndBuild { package: String },
    FetchAndBinaryOnly { package: String },
    RemovePackage { package: String },
    Upgrade { package: Option<String> },
    SyncIndex,
    ListPackages,
    InfoPackage { package: String },
    CheckManifest { manifest: String },
    NewPackageProject { name: Option<String> },
}

pub fn process_subcommand(args: &[String]) -> Result<NextCommand, Error> {
    match args[1].as_str() {
        "get" => Ok(NextCommand::FetchAndBuild {
            package: args[2].to_string(),
        }),
        "bin" => Ok(NextCommand::FetchAndBinaryOnly {
            package: args[2].to_string(),
        }),
        "rm" => Ok(NextCommand::RemovePackage {
            package: args[2].to_string(),
        }),
        "sync" => Ok(NextCommand::SyncIndex),
        "up" => Ok(NextCommand::Upgrade {
            package: Some(args[2].to_string()),
        }),
        "ls" => Ok(NextCommand::ListPackages),
        "info" => Ok(NextCommand::InfoPackage {
            package: args[2].to_string(),
        }),
        "check" => Ok(NextCommand::CheckManifest {
            manifest: args[2].to_string(),
        }),
        "new" => Ok(NextCommand::NewPackageProject {
            name: Some(args[2].to_string()),
        }),
        _ => Err(Error::InvalidSubcommand),
    }
}
