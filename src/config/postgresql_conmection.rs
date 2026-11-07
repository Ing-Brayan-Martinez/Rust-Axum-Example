use sqlx::{Error, PgPool};
use tokio::sync::OnceCell;

static DB_POOL: OnceCell<PgPool> = OnceCell::const_new();

pub async fn get_db() -> Result<&'static PgPool, Error> {
    let url = "postgres://postgres:tu_contraseña@localhost:5432/tu_base_de_datos";
    DB_POOL
        .get_or_try_init(|| async { PgPool::connect(url).await })
        .await
}
