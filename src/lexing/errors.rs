use thiserror::Error;

#[derive(Error, Debug)]
pub enum LexicalError {
    #[error("Expected 1 decimal point but got {0}")] InvalidDecimalPoint(usize),
    #[error("Unknown token: {0}")] UnknownToken(String),
}
