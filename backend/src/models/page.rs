use axum::response::{IntoResponse, Response};
use axum::Json;
use serde_derive::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PagePackage {
}

impl IntoResponse for PagePackage {
    fn into_response(self) -> Response {
        Json(self).into_response()
    }
}
