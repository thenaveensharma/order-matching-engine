use deadpool_postgres;
use std::fmt;

#[derive(Debug)]
pub enum OrderError {
    Database(tokio_postgres::Error),
    Pool(deadpool_postgres::PoolError),
}

impl fmt::Display for OrderError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            OrderError::Database(e) => write!(f, "Database error: {}", e),
            OrderError::Pool(e) => write!(f, "Connection pool error: {}", e),
        }
    }
}

impl std::error::Error for OrderError {}

impl From<tokio_postgres::Error> for OrderError {
    fn from(err: tokio_postgres::Error) -> OrderError {
        OrderError::Database(err)
    }
}

impl From<deadpool_postgres::PoolError> for OrderError {
    fn from(err: deadpool_postgres::PoolError) -> OrderError {
        OrderError::Pool(err)
    }
}
