use sqlx::{PgPool, Row};
use std::env;
use dotenv::dotenv;

pub async fn init_db() -> PgPool {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set in .env");
    PgPool::connect(&database_url).await.expect("Failed to connect to DB")
}

pub async fn add_user(
        pool: &PgPool,
        user_id: i64,
        username: &str, subscription_end_date: NaiveDate,
    ) -> Result<(), sqlx::Error> {
    sqlx::query("INSERT INTO users (user_id, username) VALUES ($1, $2) 
        ON CONFLICT (user_id) DO UPDATE SET subscription_end_date = EXCLUDED.subscription_end_date")
        .bind(user_id)
        .bind(username)
        .bind(subscription_end_date)
        .execute(pool)
        .await?;
    Ok(())
}
