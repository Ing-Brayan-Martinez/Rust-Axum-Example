use rust_axum_example::config::postgresql_conmection::get_db;

#[tokio::test]
#[ignore = "requires PostgreSQL and DATABASE_URL"]
async fn connects_to_postgresql() -> Result<(), Box<dyn std::error::Error>> {
    if std::env::var_os("DATABASE_URL").is_none() {
        match dotenvy::dotenv() {
            Ok(_) => {}
            Err(dotenvy::Error::Io(error)) if error.kind() == std::io::ErrorKind::NotFound => {}
            Err(error) => return Err(error.into()),
        }
    }

    let pool = get_db().await?;
    let result: i32 = sqlx::query_scalar("SELECT 1").fetch_one(pool).await?;

    assert_eq!(result, 1);
    Ok(())
}
