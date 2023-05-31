use ircie::system::IntoResponse;
use thiserror::Error;

pub type Result<T> = std::result::Result<T, DrugWarsError>;
pub type Error = DrugWarsError;

#[derive(Error, Debug)]
pub enum DrugWarsError {
    #[error("Io error")]
    Io(#[from] std::io::Error),
    #[error("Parse int error")]
    ParseIntError(#[from] std::num::ParseIntError),
    #[error("Parse float error")]
    ParseFloatError(#[from] std::num::ParseFloatError),
    #[error("You are already playing you donut")]
    AlreadyRegistered,
    #[error("Dealer {0} not found.")]
    DealerNotFound(String),
    #[error("Dealer {0} not available -> {1}")]
    DealerNotAvailable(String, String),
    #[error("couldn't find {0}")]
    ElementNotFound(String),
    #[error("{0} is too ambiguous. try to be more precise")]
    ElementAmbiguous(String),
    #[error("you don't have enough money you broke ass punk")]
    NotEnoughMoney,
    #[error("Invalid element {0}")]
    InvalidElement(String),
}

impl IntoResponse for DrugWarsError {
    fn response(self) -> ircie::system::Response {
        format!("{}", self).response()
    }
}
