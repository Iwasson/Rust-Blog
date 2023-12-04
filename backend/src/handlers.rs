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
use crate::models::blog::{Blog};

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
        "landing_page.html"
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

pub async fn login(
    State(database): State<Store>,
    Form(creds): Form<User>,
) -> Result<Response<Body>, AppError> {
    if creds.email.is_empty() || creds.password.is_empty() {
        return Err(AppError::MissingCredentials);
    }

    let existing_user = database.get_user(&creds.email).await?;
    let is_password_correct =
        match argon2::verify_encoded(&*existing_user.password, creds.password.as_bytes()) {
            Ok(result) => result,
            Err(_) => {
                return Err(AppError::InternalServerError);
            }
        };

    if !is_password_correct {
        return Err(AppError::InvalidPassword);
    }

    let claims = Claims {
        email: creds.email.to_owned(),
        exp: get_timestamp_after_8_hours(),
        is_admin: existing_user.is_admin,
    };

    let token = jsonwebtoken::encode(&Header::default(), &claims, &KEYS.encoding)
        .map_err(|_| AppError::MissingCredentials)?;

    let cookie = cookie::Cookie::build("jwt", token).http_only(true).finish();

    let mut response = Response::builder()
        .status(StatusCode::FOUND)
        .body(Body::empty())
        .unwrap();

    response
        .headers_mut()
        .insert(LOCATION, HeaderValue::from_static("/"));
    response.headers_mut().insert(
        SET_COOKIE,
        HeaderValue::from_str(&cookie.to_string()).unwrap(),
    );

    Ok(response)
}

pub async fn post_blog(
    State(mut am_database) : State<Store>,
    Form(blog): Form<Blog>,
) -> Result<Json<Blog>, AppError> {
    let blog = am_database
    .post_blog(blog.title, blog.email, blog.content, blog.publish_date)
    .await?;

    Ok(Json(blog))
}

pub async fn make_blog (
    State(am_database): State<Store>,
    OptionalClaims(claims): OptionalClaims,
) -> Result<Html<String>, AppError> {
    let mut context = Context::new();
    context.insert("name", "Ian");

    let template_name = if let Some(claims_data) = claims {
        error!("Setting claims and is_logged_in is TRUE now");
        context.insert("claims", &claims_data);
        context.insert("is_logged_in", &true);
        "make_blog.html"
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

pub async fn all_blogs(
    State(am_database): State<Store>,
    OptionalClaims(claims): OptionalClaims,
) -> Result<Html<String>, AppError> {
    let mut context = Context::new();
    context.insert("name", "Ian");

    let template_name = if let Some(claims_data) = claims {
        error!("Setting claims and is_logged_in is TRUE now");
        context.insert("claims", &claims_data);
        context.insert("is_logged_in", &true);
        let all_blogs = am_database.get_all_blogs().await?;
        context.insert("all_blogs", &all_blogs);
        "all_blogs.html"
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

pub async fn protected(claims: Claims) -> Result<String, AppError> {
  Ok(format!(
      "Your claim data is: {}",
      claims
  ))
}