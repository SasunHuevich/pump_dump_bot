use sqlx::PgPool;
use std::env;
use dotenv::dotenv;

pub async fn init_db() -> PgPool {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set in .env");
    PgPool::connect(&database_url).await.expect("Failed to connect to DB")
}


#[derive(sqlx::Type, Debug)]
#[sqlx(type_name = "user_role")] // имя enum в Postgres
pub enum UserRole {
    #[sqlx(rename = "user")]
    User,
    #[sqlx(rename = "admin")]
    Admin,
    #[sqlx(rename = "no_access")]
    NoAccess,
}


pub async fn add_user(
        pool: &PgPool,
        user_id: i64,
        username: &str, 
        // subscription_end_date: NaiveDate,
    ) -> Result<(), sqlx::Error> {
    sqlx::query("INSERT INTO users (user_id, username) VALUES ($1, $2) 
        ON CONFLICT (user_id) DO UPDATE SET subscription_end_date = EXCLUDED.subscription_end_date")
        .bind(user_id)
        .bind(username)
        // .bind(subscription_end_date)
        .execute(pool)
        .await?;
    Ok(())
}

pub async fn get_user_role(pool: &PgPool, user_id: i64) -> Result<UserRole, sqlx::Error> {
    let row = sqlx::query_as::<_, (UserRole,)>("SELECT role FROM users WHERE user_id = $1")
    .bind(user_id)
    .fetch_optional(pool)
    .await?;

    let role = row.map(|r| r.0).unwrap_or(UserRole::NoAccess);
    
    Ok(role)
}