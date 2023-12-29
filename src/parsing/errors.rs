use thiserror::Error;

#[derive(Error, Debug)]
pub enum SyntacticalError {
    #[error("Expected {0} but got {1}")] ExpectedToken(String, String),
}
