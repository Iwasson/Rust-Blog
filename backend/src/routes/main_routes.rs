use axum::response::Response;
use axum::routing::*;
use axum::Router;
use http::StatusCode;
use hyper::Body;
use sqlx::PgPool;

use crate::db::Store;
use crate::handlers::root;
use crate::{file_handler, handlers, layers};

pub async fn app(pool: PgPool) -> Router {
    let db = Store::with_pool(pool);

    let (cors_layer, trace_layer) = layers::get_layers();

    let static_router = Router::new()
        .route("/:filename", get(file_handler))
        .with_state(db.clone());

    Router::new()
        .nest("/static", static_router)
        .route("/", get(root))
        .route("/post_blog", post(handlers::post_blog))
        .route("/users", post(handlers::register))
        .route("/login", post(handlers::login))
        .route("/protected", get(handlers::protected))
        .route("/*_", get(handle_404))
        .layer(cors_layer)
        .layer(trace_layer)
        .with_state(db)
}

async fn handle_404() -> Response<Body> {
    Response::builder()
        .status(StatusCode::NOT_FOUND)
        .body(Body::from("The requested page could not be found"))
        .unwrap()
}
