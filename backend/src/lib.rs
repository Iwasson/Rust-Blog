use axum::body::{boxed, BoxBody};
use axum::extract::Path;
use axum::response::Response;
use std::net::{IpAddr, SocketAddr};
use std::str::FromStr;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

use crate::db::new_pool;
use crate::error::AppError;
use crate::routes::main_routes;
use dotenvy::dotenv;
use http::{Request, StatusCode, Uri};
use hyper::Body;
use tower::ServiceExt;
use tower_http::services::ServeDir;

use tracing::info;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;

pub mod db;
pub mod error;
pub mod handlers;
pub mod layers;
pub mod models;
pub mod routes;
pub mod template;

pub async fn run_backend() {
    dotenv().ok();
    init_logging();

    let addr = get_host_from_env();

    let app = main_routes::app(new_pool().await).await;

    info!("Listening...");

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

fn get_host_from_env() -> SocketAddr {
    let host = std::env::var("API_HOST").unwrap();
    let api_host = IpAddr::from_str(&host).unwrap();
    let api_port: u16 = std::env::var("API_PORT")
        .expect("Could not find API_PORT in .env file")
        .parse()
        .expect("Can't create a u16 from the given API_PORT string");

    SocketAddr::from((api_host, api_port))
}

fn init_logging() {
    // https://github.com/tokio-rs/axum/blob/main/examples/tracing-aka-logging
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| {
                "backend=trace,tower_http=debug,axum::rejection=trace".into()
            }),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();
}

pub fn get_timestamp_after_8_hours() -> u64 {
    let now = SystemTime::now();
    let since_epoch = now
        .duration_since(UNIX_EPOCH)
        .expect("Time somehow went backwards");
    // 8 hours later
    let eight_hours_from_now = since_epoch + Duration::from_secs(60 * 60 * 8);
    eight_hours_from_now.as_secs()
}

// https://benw.is/posts/serving-static-files-with-axum
pub async fn get_static_file(uri: Uri) -> Result<Response<BoxBody>, (StatusCode, String)> {
    let req = Request::builder().uri(uri).body(Body::empty()).unwrap();

    match ServeDir::new("./static").oneshot(req).await {
        Ok(res) => Ok(res.map(boxed)),
        Err(err) => Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Something went wrong: {}", err),
        )),
    }
}

pub async fn file_handler(
    Path(filename): Path<String>,
) -> Result<Response<BoxBody>, (StatusCode, String)> {
    let uri: Uri = format!("/{}", filename).parse().unwrap();
    let res = get_static_file(uri.clone()).await?;

    if res.status() == StatusCode::NOT_FOUND {
        match format!("{}.html", uri).parse() {
            Ok(uri_html) => get_static_file(uri_html).await,
            Err(_) => Err((StatusCode::INTERNAL_SERVER_ERROR, "Invalid URI".to_string())),
        }
    } else {
        Ok(res)
    }
}

pub type AppResult<T> = Result<T, AppError>;

/// Basic macro to create a newtype for a database ID.
#[macro_export]
macro_rules! make_db_id {
    ($name:ident) => {
        use derive_more::Display;

        paste::paste! {
            #[derive(
                Clone,
                Copy,
                Debug,
                sqlx::Type,
                Display,
                derive_more::Deref,
                PartialEq,
                Eq,
                Hash,
                serde_derive::Serialize,
                serde_derive::Deserialize,
            )]
            pub struct $name(pub i32);

            impl From<i32> for $name {
                fn from(value: i32) -> Self {
                    $name(value)
                }
            }

            impl From<$name> for i32 {
                fn from(value: $name) -> Self {
                    value.0
                }
            }

            pub trait [<Into $name>] {
                fn into_id(self) -> $name;
            }

            impl [<Into $name>] for i32 {
                fn into_id(self) -> $name {
                    $name::from(self)
                }
            }

            impl [<Into $name>] for $name {
                fn into_id(self) -> $name {
                    self
                }
            }
        }
    };
}
