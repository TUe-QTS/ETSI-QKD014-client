use std::fmt;

pub(crate) type BoxError = Box<dyn std::error::Error>;

#[derive(Debug)]
pub enum ErrorType {
    InvalidHost,
    InvalidArgument,
    ConnectionError,
    InvalidResponse,
}
#[derive(Debug)]
pub struct Error {
    pub msg: String,
    pub kind: ErrorType,
    pub source: Option<BoxError>,
}

impl Error {
    pub(crate) fn new(
        msg: String,
        kind: ErrorType,
        source: Option<Box<dyn std::error::Error>>,
    ) -> Error {
        Error { msg, kind, source }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let caused_by = match self.source {
            Some(ref e) => format!("\n\nCaused by: {:#?}", e),
            None => "".to_string(),
        };
        write!(f, "{}{}", self.msg, caused_by)
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        self.source.as_deref()
    }
}
