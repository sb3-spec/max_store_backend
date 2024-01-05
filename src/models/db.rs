use super::Error;
use sqlx::PgPool;
use std::env;

pub async fn connect_to_db() -> Result<PgPool, Error> {
    let pool = PgPool::connect(&env::var("DATABASE_URL")?).await?;

    match sqlx::migrate!().run(&pool).await {
        Ok(_) => println!("Migrations successfully applied"),
        Err(e) => println!("Error applying migrations: {:?}", e),
    }

    Ok(pool)
}
