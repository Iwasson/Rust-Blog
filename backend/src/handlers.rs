use argon2::Config;
use axum::extract::{Path, Query, State};
use axum::response::{Html, Response};
use axum::{Form, Json};
use http::header::{LOCATION, SET_COOKIE};
use http::{HeaderValue, StatusCode};
use hyper::Body;
use jsonwebtoken::Header;
use serde_json::Value;
use tera::Context;
use tracing::error;

use crate::db::Store;
use crate::error::AppError;
use crate::get_timestamp_after_8_hours;
use crate::models::users::{Claims, OptionalClaims, User, UserSignup, KEYS};

use crate::template::TEMPLATES;

#[allow(dead_code)]
pub async fn root(
    State(am_database): State<Store>,
    OptionalClaims(claims): OptionalClaims,
) -> Result<Html<String>, AppError> {
    let mut context = Context::new();
    context.insert("name", "Ian");

    let template_name = if let Some(claims_data) = claims {
        error!("Setting claims and is_logged_in is TRUE now");
        context.insert("claims", &claims_data);
        context.insert("is_logged_in", &true);
        "pages.html"
    } else {
        error!("is_logged_in is FALSE now");
        context.insert("is_logged_in", &false);
        "index.html"
    };

    let rendered = TEMPLATES
        .render(template_name, &context)
        .unwrap_or_else(|err| {
            error!("Template rendering error: {}", err);
            panic!()
        });
    Ok(Html(rendered))
}

pub async fn register(
  State(database): State<Store>,
  Json(mut credentials): Json<UserSignup>,
) -> Result<Json<Value>, AppError> {
  if credentials.email.is_empty() || credentials.password.is_empty() {
      return Err(AppError::MissingCredentials);
  }

  if credentials.password != credentials.confirm_password {
      return Err(AppError::MissingCredentials);
  }

  let existing_user = database.get_user(&credentials.email).await;

  if let Ok(_) = existing_user {
      return Err(AppError::UserAlreadyExists);
  }

  let hash_config = Config::default();
  let salt = std::env::var("SALT").expect("Missing SALT");
  let hashed_password = match argon2::hash_encoded(
      credentials.password.as_bytes(),
      salt.as_bytes(),
      &hash_config,
  ) {
      Ok(result) => result,
      Err(_) => {
          return Err(AppError::Any(anyhow::anyhow!("Password hashing failed")));
      }
  };

  credentials.password = hashed_password;

  let new_user = database.create_user(credentials).await?;
  Ok(new_user)
}

pub async fn protected(claims: Claims) -> Result<String, AppError> {
  Ok(format!(
      "Your claim data is: {}",
      claims
  ))
}