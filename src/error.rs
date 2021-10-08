use std::fmt;

#[derive(Debug)]
pub enum Error {
  Io(std::io::Error),
  Utf8(std::str::Utf8Error),
  FromUtf8(std::string::FromUtf8Error),
  Yaml(serde_yaml::Error),
  Template(tera::Error),
}

impl fmt::Display for Error {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "Error!")
  }
}

impl std::convert::From<std::io::Error> for Error {
  fn from(error: std::io::Error) -> Self {
    Error::Io(error)
  }
}

impl std::convert::From<std::str::Utf8Error> for Error {
  fn from(error: std::str::Utf8Error) -> Self {
    Error::Utf8(error)
  }
}

impl std::convert::From<std::string::FromUtf8Error> for Error {
  fn from(error: std::string::FromUtf8Error) -> Self {
    Error::FromUtf8(error)
  }
}

impl std::convert::From<serde_yaml::Error> for Error {
  fn from(error: serde_yaml::Error) -> Self {
    Error::Yaml(error)
  }
}

impl std::convert::From<tera::Error> for Error {
  fn from(error: tera::Error) -> Self {
    Error::Template(error)
  }
}
