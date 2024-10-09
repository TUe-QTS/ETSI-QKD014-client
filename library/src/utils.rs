use crate::error::ErrorType::InvalidArgument;
use crate::Error;
use std::fs;
use std::path::PathBuf;

pub fn read_file(path: &PathBuf) -> Result<Vec<u8>, Error> {
    fs::read(path).map_err(|e| {
        Error::new(
            format!("Error reading {path:#?}"),
            InvalidArgument,
            Some(Box::new(e)),
        )
    })
}
