use sqlx::PgPool;
use std::env;
use dotenv::dotenv;
use serde::{Serialize, Deserialize};

pub async fn init_db() -> PgPool {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set in .env");
    PgPool::connect(&database_url).await.expect("Failed to connect to DB")
}


#[derive(sqlx::Type, Debug, PartialEq)]
#[sqlx(type_name = "user_role")]
pub enum UserRole {
    #[sqlx(rename = "user")]
    User,
    #[sqlx(rename = "admin")]
    Admin,
    #[sqlx(rename = "no_access")]
    NoAccess,
}

#[derive(sqlx::Type, Debug, Clone, Serialize, Deserialize, PartialEq)]
#[sqlx(type_name = "user_state")]
pub enum UserState {
    #[sqlx(rename = "main_menu")]
    MainMenu,

    #[sqlx(rename = "add_user_menu")]
    AddUserMenu,

    #[sqlx(rename = "add_user_button")]
    AddUserButton,
}


pub async fn add_user(
        pool: &PgPool,
        user_id: i64
    ) -> Result<(), sqlx::Error> {
    sqlx::query("INSERT INTO users (user_id) VALUES ($1) 
        ON CONFLICT (user_id) DO NOTHING")
        .bind(user_id)
        .execute(pool)
        .await?;

    sqlx::query("INSERT INTO user_settings (user_id) VALUES ($1) 
        ON CONFLICT (user_id) DO NOTHING")
        .bind(user_id)
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

pub async fn set_user_state(pool: &PgPool, user_id: i64, state: &UserState) -> Result<(), sqlx::Error> {
    sqlx::query("INSERT INTO user_states (user_id, state) VALUES ($1, $2) 
        ON CONFLICT (user_id) DO UPDATE SET state = EXCLUDED.state")
        .bind(user_id)
        .bind(state)
        .execute(pool)
        .await?;
    Ok(())
}

pub async fn get_user_state(pool: &PgPool, user_id: i64) -> Result<UserState, sqlx::Error> {
    let row = sqlx::query_as::<_, (UserState,)>("SELECT state FROM user_states WHERE user_id = $1")
        .bind(user_id)
        .fetch_optional(pool)
        .await?;

    let state = row.map(|r| r.0).unwrap_or(UserState::MainMenu);

    Ok(state)
}