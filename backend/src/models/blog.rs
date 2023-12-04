use serde_derive::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Blog {
    pub title: String,
    pub email: String,
    pub content: String,
    pub publish_date: String,
}

impl Blog {
  #[allow(dead_code)]
  pub fn new(title: String, email: String, content: String, publish_date: String) -> Self {
    Blog {
      title,
      email,
      content,
      publish_date
    }
  }
}
