use crate::models::prelude::*;

#[derive(Debug)]
pub struct Canvas {
    base_url: String,
    token: String,
}

impl Canvas {
    /// Create a new instance of the Canvas API.
    pub fn new(base_url: String, token: String) -> Self {
        Self { base_url, token }
    }

    pub(crate) fn add_url_prefix(&self, url: &str) -> String {
        format!("{}/api/v1/{}", self.base_url, url)
    }

    pub(crate) fn get_request(&self, url: String) -> actix_web::client::ClientRequest {
        actix_web::client::Client::new()
            .get(&url)
            .bearer_auth(self.token.as_str())
    }
}
