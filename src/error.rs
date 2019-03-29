#[derive(Debug)]
pub enum Error {
    SerdeError(serde_json::Error),
    Malformed(String),
}
