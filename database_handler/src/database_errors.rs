use diesel::result::Error as DieselError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum DbError {
    #[error("R2D2 error | Couldn't connect to database")]
    ConnectionError(#[from] r2d2::Error),
    #[error("Diesel error | Couldn't connect to database")]
    InvalidCString(String),
    #[error("Diesel error | Couldn't connect to database")]
    DatabaseError(String),
    #[error("Diesel error | Couldn't connect to database")]
    NotFound(String),
    #[error("Diesel error | Couldn't connect to database")]
    QueryBuilderError(String),
    #[error("Diesel error | Couldn't connect to database")]
    DeserializationError(String),
    #[error("Diesel error | Couldn't connect to database")]
    SerializationError(String),
    #[error("Diesel error | Couldn't connect to database")]
    RollbackTransaction(String),
    #[error("Diesel error | Couldn't connect to database")]
    AlreadyInTransaction(String),
    #[error("Diesel error | Couldn't connect to database")]
    Nonexhaustive(String),
}

impl From<DieselError> for DbError {
    fn from(error: DieselError) -> Self {
        match error {
            DieselError::InvalidCString(e) => DbError::InvalidCString(e.to_string()),
            DieselError::DatabaseError(_, e) => DbError::DatabaseError(e.message().to_string()),
            DieselError::NotFound => DbError::NotFound("Not found".to_string()),
            DieselError::QueryBuilderError(e) => DbError::QueryBuilderError(e.to_string()),
            DieselError::DeserializationError(e) => DbError::DeserializationError(e.to_string()),
            DieselError::SerializationError(e) => DbError::SerializationError(e.to_string()),
            DieselError::RollbackTransaction => DbError::RollbackTransaction("Rollback occurred".to_string()),
            DieselError::AlreadyInTransaction => DbError::AlreadyInTransaction("Transaction in progress".to_string()),
            DieselError::__Nonexhaustive => DbError::Nonexhaustive("Nonexhaustive".to_string()),
        }
    }
}