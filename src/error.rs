use ircie::system::{IntoResponse, Response};
use thiserror::Error;

pub type Result<T> = std::result::Result<T, DrugWarsError>;
pub type Error = DrugWarsError;

#[derive(Error, Debug)]
pub enum DrugWarsError {
    #[error("Io error")]
    Io(#[from] std::io::Error),
    #[error("{0}: You are already playing you donut")]
    AlreadyRegistered(String),
    #[error("Dealer {0} not found.")]
    DealerNotFound(String),
}

impl IntoResponse for DrugWarsError {
    fn response(self) -> ircie::system::Response {
        Response(Some(vec![format!("{}", self)]))
    }
}
