//! Model for managing files with Canvas.

use anyhow::anyhow;
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
    pub async fn download(&self, canvas: &CanvasInformation<'_>, path: &str) -> anyhow::Result<()> {
        let url = self
            .url
            .clone()
            .ok_or_else(|| anyhow!("File url not set"))?;
        let name = self
            .filename
            .clone()
            .ok_or_else(|| anyhow!("File name not set"))?;

        let mut resp = canvas.get_request(url).send().await?.bytes().await?;

        std::fs::write(format!("{path}/{name}"), resp)?;

        Ok(())
    }

    #[cfg(feature = "blocking")]
    pub fn download(&self, canvas: &CanvasInformation<'_>, path: &str) -> anyhow::Result<()> {
        let url = self
            .url
            .clone()
            .ok_or_else(|| anyhow!("File url not set"))?;
        let name = self
            .filename
            .clone()
            .ok_or_else(|| anyhow!("File name not set"))?;

        let mut resp = canvas.get_request(url).send()?.bytes()?;

        std::fs::write(format!("{path}/{name}"), resp)?;

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
