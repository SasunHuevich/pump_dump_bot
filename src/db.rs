use sqlx::PgPool;
use std::env;
use dotenv::dotenv;
use serde::{Serialize, Deserialize};
use chrono::NaiveDate;
use sqlx::Row;
use chrono::NaiveDateTime;
//use bigdecimal::BigDecimal;


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

    #[sqlx(rename = "add_admin_button")]
    AddAdminButton,

    #[sqlx(rename = "delete_user_button")]
    DeleteUserButton,

    #[sqlx(rename = "choosing_exchange_menu")]
    ChoosingExchangeMenu,
}


pub async fn add_user(
        pool: &PgPool,
        user_id: i64,
        subscription_end_date: Option<NaiveDate>
    ) -> Result<(), sqlx::Error> {
    sqlx::query("INSERT INTO users (user_id, subscription_end_date) VALUES ($1, $2) 
        ON CONFLICT (user_id) DO UPDATE SET subscription_end_date = EXCLUDED.subscription_end_date")
        .bind(user_id)
        .bind(subscription_end_date)
        .execute(pool)
        .await?;

    sqlx::query("INSERT INTO user_settings (user_id) VALUES ($1) 
        ON CONFLICT (user_id) DO NOTHING")
        .bind(user_id)
        .execute(pool)
        .await?;

    Ok(())
}

pub async fn add_admin(pool: &PgPool, user_id: i64) -> Result<(), sqlx::Error> {
    add_user(pool, user_id, None).await?;

    sqlx::query("UPDATE users SET role = 'admin' WHERE user_id = $1")
        .bind(user_id)
        .execute(pool)
        .await?;

    Ok(())
}

pub async fn delete_user(pool: &PgPool, user_id: i64) -> Result<(), sqlx::Error> {
    sqlx::query("DELETE FROM user_settings WHERE user_id = $1")
        .bind(user_id)
        .execute(pool)
        .await?;

    sqlx::query("DELETE FROM users WHERE user_id = $1")
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


pub async fn switch_exchange_state (
    pool: &PgPool,
    user_id: i64,
    exchange: &str
) -> Result<(), sqlx::Error> {
    let query = format!(
        "UPDATE user_settings SET {col} = NOT {col} WHERE user_id = $1",
        col = exchange
    );

     sqlx::query(&query)
        .bind(user_id)
        .execute(pool)
        .await?;
    Ok(())
}

#[derive(sqlx::FromRow)]
pub struct EnableExchanges {
    pub binance: bool,
    pub bybit: bool,
    pub bingx: bool,
}

pub async fn get_exchanges(pool: &PgPool, user_id: i64) -> Result<EnableExchanges, sqlx::Error> {
    sqlx::query_as::<_, EnableExchanges>(
        "SELECT binance, bybit, bingx FROM user_settings WHERE user_id = $1"
    )
    .bind(user_id)
    .fetch_one(pool)
    .await
}

pub struct UnifiedPrice {
    pub currency: String,
    pub price: f64,
    pub exchange: String,
    pub timestamp: i64,
}

pub async fn insert_currency_prices_bulk(pool: &PgPool, list: &[UnifiedPrice]) -> Result<(), sqlx::Error> {
    let mut tx = pool.begin().await?;

    for p in list {
        sqlx::query(
            "INSERT INTO currency_prices (currency, price, exchange, created_at)
            VALUES ($1, $2, $3, to_timestamp($4 / 1000.0))"
        )
        .bind(&p.currency)
        .bind(p.price)
        .bind(&p.exchange)
        .bind(p.timestamp)
        .execute(&mut *tx)
        .await?;
    }

    tx.commit().await?;
    Ok(())
}

pub async fn delete_old_currency_prices(pool: &PgPool) -> Result<(), sqlx::Error> {
    sqlx::query(
        "DELETE FROM currency_prices WHERE created_at < NOW() - INTERVAL '30 minutes'"
    )
    .execute(pool)
    .await?;

    Ok(())
}

#[derive(sqlx::FromRow)]
pub struct PriceRow {
    pub currency: String,
    pub price: f64,
    pub exchange: String,
    pub created_at: NaiveDateTime,
}

pub async fn get_prices_sort_created(
    pool: &PgPool,
    max_interval_minutes: i32,
) -> Result<Vec<PriceRow>, sqlx::Error> {
    let rows = sqlx::query_as::<_, PriceRow>(
        "SELECT currency, price, exchange, created_at
         FROM currency_prices
         WHERE created_at >= NOW() -  ($1 * INTERVAL '1 minute')
         ORDER BY created_at ASC"
    )
    .bind(max_interval_minutes)
    .fetch_all(pool)
    .await?;

    Ok(rows)
}

#[derive(sqlx::FromRow)]
pub struct UserSettings {
    pub user_id: i64,

    pub binance: bool,
    pub bybit: bool,
    pub bingx: bool,

    pub long_percent: f64,
    pub long_minutes: i32,

    pub short_percent: f64,
    pub short_minutes: i32,

    pub drop_percent: f64,
    pub drop_minutes: i32,

    pub subscription_end_date: Option<chrono::NaiveDateTime>,
}

pub async fn get_user_settings(pool: &PgPool) -> Result<Vec<UserSettings>, sqlx::Error> {
    sqlx::query_as::<_, UserSettings>(
        "
        SELECT 
            us.user_id,
            us.binance,
            us.bybit,
            us.bingx,
            us.long_percent,
            us.long_minutes,
            us.short_percent,
            us.short_minutes,
            us.drop_percent,
            us.drop_minutes,
            u.subscription_end_date
        FROM user_settings us
        LEFT JOIN users u ON u.user_id = us.user_id
        "
    )
    .fetch_all(pool)
    .await
}

pub async fn insert_notification(
    pool: &PgPool,
    user_id: i64,
    currency: &str,
    notification_type: &str,
) ->Result<(), sqlx::Error> {
    sqlx::query(
        "INSERT INTO user_notifications (user_id, currency, notification_type)
        VALUES ($1, $2, $3::notification_enum)"
    )
    .bind(user_id)
    .bind(currency)
    .bind(notification_type)
    .execute(pool)
    .await?;

    Ok(())
}


pub async fn count_notifications_for_user(
    pool: &PgPool,
    user_id: i64,
    currency: &str,
    notif_type: &str
) -> Result<i64, sqlx::Error> {
    let row = sqlx::query(
        "
        SELECT COUNT(*) AS count
        FROM user_notifications
        WHERE user_id = $1
          AND currency = $2
          AND notification_type = $3::notification_enum
          AND created_at >= NOW() - INTERVAL '24 hours'
        "
    )
    .bind(user_id)
    .bind(currency)
    .bind(notif_type)
    .fetch_one(pool)
    .await?;

    let count: i64 = row.try_get("count")?;

    Ok(count)
}

pub async fn delete_old_user_notifications(pool: &PgPool) -> Result<(), sqlx::Error> {
    sqlx::query(
        "DELETE FROM user_notifications WHERE created_at < NOW() - INTERVAL '24 hours'"
    )
    .execute(pool)
    .await?;

    Ok(())
}

// TODO: все функции в этом файле наверное тоже как-то логировать надо