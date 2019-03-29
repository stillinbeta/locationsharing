extern crate reqwest;
extern crate serde_json;

#[derive(Debug)]
pub enum Error {
    SerdeError(serde_json::Error),
    Malformed(String),
    ReqwestError(reqwest::Error),
    IOError(std::io::Error),
    VarError(std::env::VarError),
}

impl From<reqwest::Error> for Error {
    fn from(source: reqwest::Error) -> Self {
        Error::ReqwestError(source)
    }
}

impl From<serde_json::Error> for Error {
    fn from(source: serde_json::Error) -> Self {
        Error::SerdeError(source)
    }
}

impl From<std::io::Error> for Error {
    fn from(source: std::io::Error) -> Self {
        Error::IOError(source)
    }
}

impl From<std::env::VarError> for Error {
    fn from(source: std::env::VarError) -> Self {
        Error::VarError(source)
    }
}
