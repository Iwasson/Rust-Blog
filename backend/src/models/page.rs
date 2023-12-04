use axum::response::{IntoResponse, Response};
use axum::Json;
use serde_derive::{Deserialize, Serialize};
use crate::models::blog::{Blog};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PagePackage {
}

#[derive(Serialize, Deserialize, Debug)]
pub struct BlogPage {
    pub blog_page: Blog,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AllPages {
    pub blog_pages: Vec<Blog>,
}

impl IntoResponse for PagePackage {
    fn into_response(self) -> Response {
        Json(self).into_response()
    }
}
