use crate::canvas::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct User {
    pub id: usize,
    pub name: Option<String>,
    pub created_at: Option<String>,
    pub sortable_name: Option<String>,
    pub short_name: Option<String>,
    pub sis_user_id: Option<String>,
    pub integration_id: Option<String>,
    pub login_id: Option<String>,
    pub email: Option<String>,
    pub avatar_url: Option<String>,
}
