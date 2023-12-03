use axum::Json;
use reqwest::Client;
use serde_json::Value;
use std::sync::{Arc, Mutex};

use sqlx::postgres::PgPoolOptions;
use sqlx::{PgPool, Row};
use tracing::info;

use crate::error::AppError;
use crate::models::page::{PagePackage};
use crate::models::users::{User, UserSignup};

#[derive(Clone)]
pub struct Store {
    pub conn_pool: PgPool,
}

pub async fn new_pool() -> PgPool {
    let db_url = std::env::var("DATABASE_URL").unwrap();
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&db_url)
        .await
        .unwrap();
    // Run migrations
    info!("Performing migrations");
    sqlx::migrate!("./migrations")
        .run(&pool)
        .await
        .unwrap();
    info!("Finished migrating");

    pool
}

impl Store {
  pub fn with_pool(pool: PgPool) -> Self {
      Self {
          conn_pool: pool,
      }
  }

  pub async fn test_database(&self) -> Result<(), sqlx::Error> {
    let row: (i64,) = sqlx::query_as("SELECT $1")
        .bind(150_i64)
        .fetch_one(&self.conn_pool)
        .await?;

    info!("{}", &row.0);

    assert_eq!(row.0, 150);
    Ok(())
  }

  pub async fn get_user(&self, email: &str) -> Result<User, AppError> {
    let user = sqlx::query_as::<_, User>(
        r#"
            SELECT email, password, is_admin FROM users WHERE email = $1
        "#,
    )
    .bind(email)
    .fetch_one(&self.conn_pool)
    .await?;

    Ok(user)
  }

  pub async fn create_user(&self, user: UserSignup) -> Result<Json<Value>, AppError> {
    // TODO: Encrypt/bcrypt user passwords
    let result = sqlx::query("INSERT INTO users(email, password, is_admin) values ($1, $2, $3)")
        .bind(&user.email)
        .bind(&user.password)
        .bind(&user.is_admin)
        .execute(&self.conn_pool)
        .await
        .map_err(|_| AppError::InternalServerError)?;

    if result.rows_affected() < 1 {
        Err(AppError::InternalServerError)
    } else {
        Ok(Json(
            serde_json::json!({"message": "User created successfully!"}),
        ))
    }
  }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
