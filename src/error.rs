use std::fmt;

pub enum Error {
    InvalidSubcommand,
    InvalidArguments,
    IoError,
    InvalidManifest,
    BuilderError,
    ExternalError,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::InvalidSubcommand => write!(f, "rei: Invalid Subcommand"),
            Error::InvalidArguments => write!(f, "rei: Invalid Arguments"),
            Error::IoError => write!(f, "rei: Io Error"),
            Error::InvalidManifest => write!(f, "rei: Invalid Manifest "),
            Error::BuilderError => write!(f, "rei: Builder Error"),
            Error::ExternalError => write!(f, "rei: External Error"),
        }
    }
}
