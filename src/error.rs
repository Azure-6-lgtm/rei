use std::fmt;

pub enum Error {
    InvalidSubcommand,
    InvalidArguments,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::InvalidSubcommand => write!(f, "rei: Invalid Subcommand"),
            Error::InvalidArguments => write!(f, "rei: Invalid Arguments"),
        }
    }
}
