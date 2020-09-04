//! Model for managing files with Canvas.

use serde::{Deserialize, Serialize};

use crate::canvas::CanvasInformation;
use crate::models::prelude::*;
use crate::parameters::*;
use crate::requests::*;

#[derive(Debug, Deserialize, Serialize)]
pub struct File {
    pub id: usize,
    pub uuid: Option<String>,
    pub folder_id: Option<usize>,
    pub display_name: Option<String>,
    pub filename: Option<String>,
    #[serde(rename(deserialize = "content-type"))]
    pub content_type: Option<String>,
    pub url: Option<String>,
    pub size: Option<usize>,
    pub created_at: Option<String>,
    pub updated_at: Option<String>,
    pub unlock_at: Option<String>,
    pub locked: Option<bool>,
    pub hidden: Option<bool>,
    pub thumbnail_url: Option<String>,
    pub modified_at: Option<String>,
    pub mime_class: Option<String>,
    pub media_entry_id: Option<String>,
    pub locked_for_user: Option<bool>,
    pub lock_info: Option<String>,
    pub lock_explanation: Option<String>,
    pub preview_url: Option<String>,
}

impl File {
    #[cfg(not(feature = "blocking"))]
    pub async fn download(
        &self,
        canvas: &CanvasInformation<'_>,
        path: &str,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let url = self.url.clone().unwrap();
        let name = self.filename.clone().unwrap();

        let mut resp = canvas
            .get_request(url)
            .send()
            .await
            .unwrap()
            .bytes()
            .await
            .unwrap();

        std::fs::write(format!("{}/{}", path, name), resp).expect("failed to copy the content");

        Ok(())
    }

    #[cfg(feature = "blocking")]
    pub fn download(
        &self,
        canvas: &CanvasInformation<'_>,
        path: &str,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let url = self.url.clone().unwrap();
        let name = self.filename.clone().unwrap();

        let mut resp = canvas.get_request(url).send().unwrap().bytes().unwrap();

        std::fs::write(format!("{}/{}", path, name), resp).expect("failed to copy the content");

        Ok(())
    }

    api_todo! {
        /// Delete this file
        delete(self)
    }

    api_todo! {
        /// Download the contents of this file
        get_contents(self)
    }
}
