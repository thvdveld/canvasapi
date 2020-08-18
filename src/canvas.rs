use crate::models::prelude::*;
use crate::requests::*;

#[derive(Debug)]
pub struct CanvasInformation {
    base_url: String,
    token: String,
}

impl CanvasInformation {
    /// Create a new instance of the Canvas API.
    pub fn new(base_url: String, token: String) -> Self {
        Self { base_url, token }
    }

    pub(crate) fn add_url_prefix(&self, url: &str) -> String {
        format!("{}/api/v1/{}", self.base_url, url)
    }

    /// Create a client for a get request.
    /// This adds the url and the token.
    pub(crate) fn get_request(&self, url: String) -> reqwest::RequestBuilder {
        reqwest::Client::new()
            .get(&url)
            .bearer_auth(self.token.as_str())
    }

    pub(crate) fn get_token(&self) -> &str {
        &self.token
    }
}
