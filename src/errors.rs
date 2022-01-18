use std::{error::Error, fmt, io};

use epub_builder::Error as EpubError;
use imagesize::ImageError;
use log::SetLoggerError;

pub type CliResult<T> = std::result::Result<T, CliError>;

#[derive(Debug)]
pub enum ErrorKind {
    Epub(EpubError),
    Log(SetLoggerError),
    Io(io::Error),
    Image(ImageError),
    Msg(String),
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
            Image(err) => writeln!(f, "Image error: {}", err)?,
            Msg(msg) => writeln!(f, "{}", msg)?,
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
            Image(err) => Some(err),
            Msg(_) => None,
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

impl From<ImageError> for CliError {
    fn from(error: ImageError) -> CliError {
        CliError::with_kind(ErrorKind::Image(error))
    }
}

impl From<String> for CliError {
    fn from(message: String) -> CliError {
        CliError::with_kind(ErrorKind::Msg(message))
    }
}

pub trait ResultExt {
    type OkValue;
    fn context<S: ToString>(self, message: S) -> Result<Self::OkValue, CliError>;
}

impl<T, E> ResultExt for Result<T, E>
where
    E: Into<CliError>,
{
    type OkValue = T;

    fn context<S: ToString>(self, message: S) -> Result<Self::OkValue, CliError> {
        match self {
            Ok(value) => Ok(value),
            Err(err) => Err(err.into().context(message)),
        }
    }
}
