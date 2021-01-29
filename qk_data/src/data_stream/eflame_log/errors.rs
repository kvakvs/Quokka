use std::io;

#[derive(Debug)]
pub enum EflameError {
  IOError(io::Error),
  ParseError(String),
}

impl From<io::Error> for EflameError {
  fn from(e: io::Error) -> Self {
    Self::IOError(e)
  }
}

impl From<nom::Err<nom::error::Error<&str>>> for EflameError {
  fn from(e: nom::Err<nom::error::Error<&str>>) -> Self {
    Self::ParseError(e.to_string())
  }
}