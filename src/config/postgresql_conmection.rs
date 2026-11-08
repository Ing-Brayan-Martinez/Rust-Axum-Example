use sqlx::{Error, PgPool};
use tokio::sync::OnceCell;

static DB_POOL: OnceCell<PgPool> = OnceCell::const_new();

pub async fn get_db() -> Result<&'static PgPool, Error> {
    let url = std::env::var("DATABASE_URL")
        .map_err(|error| Error::Configuration(Box::new(error)))?;
    
    DB_POOL
        .get_or_try_init(|| async { PgPool::connect(&url).await })
        .await
}
