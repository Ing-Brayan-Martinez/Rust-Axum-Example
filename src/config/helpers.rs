pub enum ServiceError {
    NotFound,
    Database(sqlx::Error),
}

impl From<sqlx::Error> for ServiceError {
    fn from(error: sqlx::Error) -> Self {
        Self::Database(error)
    }
}
