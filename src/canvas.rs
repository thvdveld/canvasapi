//! Canvas structs that are not part of the API, but are needed for the library.

use crate::models::prelude::*;
use crate::requests::*;

/// Contains information about the server URL and the API access token.
///
/// # Example
/// ```
/// # use canvasapi::canvas::CanvasInformation;
/// let canvas = CanvasInformation::new("https://canvas.test.be", "MY_TOKEN");
/// ```
#[derive(Debug)]
pub struct CanvasInformation<'i> {
    base_url: &'i str,
    token: &'i str,
}

impl<'i> CanvasInformation<'i> {
    /// Create a new instance of the Canvas API.
    pub fn new(base_url: &'i str, token: &'i str) -> Self {
        Self { base_url, token }
    }

    pub(crate) fn add_url_prefix(&self, url: &str) -> String {
        format!("{}/api/v1/{}", self.base_url, url)
    }

    /// Create a client for a get request.
    /// This adds the url and the token.
    pub(crate) fn get_request(&self, url: String) -> reqwest::RequestBuilder {
        reqwest::Client::new().get(&url).bearer_auth(self.token)
    }

    pub(crate) fn get_token(&self) -> &str {
        &self.token
    }
}
