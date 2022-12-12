use crate::models::*;
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Deserialize, Serialize)]
pub struct Todo {
    pub type_field: Option<String>,
    pub assignment: Option<assignment::Assignment>,
    pub quiz: Option<quiz::Quiz>,
    pub ignore: Value,
    pub ignore_permanently: Option<String>,
    pub html_url: Option<String>,
    pub needs_grading_count: Option<i64>,
    pub context_type: Option<String>,
    pub course_id: Option<i64>,
    pub group_id: Option<usize>,
}
