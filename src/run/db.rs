use std::str::FromStr;

use sqlx::{sqlite::SqliteConnectOptions, SqlitePool};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error(transparent)]
    Sqlx(#[from] sqlx::Error),
    #[error(transparent)]
    Migrate(#[from] sqlx::migrate::MigrateError),
}

pub struct Db {
    pool: SqlitePool,
}

impl Db {
    pub async fn new(database_url: &str) -> Result<Self, Error> {
        let pool = SqlitePool::connect_with(
            SqliteConnectOptions::from_str(database_url)?.create_if_missing(true),
        )
        .await?;

        sqlx::migrate!("./migrations").run(&pool).await?;

        Ok(Self { pool })
    }

    pub async fn record_query(&mut self, query: &telegram_bot::InlineQuery) -> Result<(), Error> {
        sqlx::query(
            "
            INSERT INTO users
            VALUES (
                $1, -- id
                $2, -- first_name
                $3, -- last_name
                $4, -- username
                $5, -- language_code
                $6, -- request_count
                $7, -- first_seen
                $8  -- last_seen
            )
            ON CONFLICT (id) DO
            UPDATE SET first_name = $2,
                       last_name = $3,
                       username = $4,
                       language_code = $5,
                       request_count = request_count + 1,
                       last_seen = $8
            ",
        )
        .bind(unsafe { std::mem::transmute::<_, i64>(query.from.id) })
        .bind(&query.from.first_name)
        .bind(&query.from.last_name)
        .bind(&query.from.username)
        .bind(&query.from.language_code)
        .bind(1)
        .bind(chrono::Utc::now())
        .bind(chrono::Utc::now())
        .execute(&mut self.pool.acquire().await?)
        .await?;

        Ok(())
    }
}
