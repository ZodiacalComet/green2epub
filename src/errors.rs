use std::{error::Error, fmt, io};

use epub_builder::Error as EpubError;
use log::SetLoggerError;

pub type Result<T> = std::result::Result<T, CliError>;

#[derive(Debug)]
pub enum ErrorKind {
    Epub(EpubError),
    Log(SetLoggerError),
    Io(io::Error),
}

#[derive(Debug)]
pub struct CliError {
    pub kind: ErrorKind,
    context: Option<String>,
}

impl CliError {
    pub fn with_kind(kind: ErrorKind) -> Self {
        Self {
            kind,
            context: None,
        }
    }

    pub fn context<S>(mut self, context: S) -> Self
    where
        S: ToString,
    {
        self.context = Some(context.to_string());
        self
    }
}

impl fmt::Display for CliError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use ErrorKind::*;

        match &self.kind {
            Epub(err) => writeln!(f, "Epub error: {}", err)?,
            Log(err) => writeln!(f, "Log error: {}", err)?,
            Io(err) => writeln!(f, "IO error: {}", err)?,
        };

        if let Some(context) = &self.context {
            writeln!(f, "Context: {}", context)?;
        }

        Ok(())
    }
}

impl Error for CliError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        use ErrorKind::*;

        match &self.kind {
            Epub(err) => Some(err),
            Log(err) => Some(err),
            Io(err) => Some(err),
        }
    }
}

impl From<EpubError> for CliError {
    fn from(error: EpubError) -> CliError {
        CliError::with_kind(ErrorKind::Epub(error))
    }
}

impl From<SetLoggerError> for CliError {
    fn from(error: SetLoggerError) -> CliError {
        CliError::with_kind(ErrorKind::Log(error))
    }
}

impl From<io::Error> for CliError {
    fn from(error: io::Error) -> CliError {
        CliError::with_kind(ErrorKind::Io(error))
    }
}
