use serde::{Deserialize, Serialize};

use crate::canvas::CanvasInformation;
use crate::models::prelude::*;
use crate::parameters::*;
use crate::requests::*;

#[derive(Debug, Deserialize, Serialize)]
pub struct Submission {
    pub assignment_id: Option<usize>,
    pub assignment: Option<usize>,
    pub course: Option<usize>,
    pub attempt: Option<usize>,
    pub body: Option<String>,
    pub grade: Option<String>,
    pub grade_matches_current_submission: Option<bool>,
    pub html_url: Option<String>,
    pub preview_url: Option<String>,
    pub score: Option<f64>,
    pub submission_comments: Option<Vec<String>>,
    pub submission_type: Option<String>,
    pub submitted_at: Option<String>,
    pub url: Option<String>,
    pub user_id: Option<usize>,
    pub grader_id: Option<usize>,
    pub graded_at: Option<String>,
    pub user: Option<usize>,
    pub late: bool,
    pub assignment_visible: Option<bool>,
    pub excused: Option<bool>,
    pub missing: Option<bool>,
    pub late_policy_status: Option<String>,
    pub points_deducted: Option<f64>,
    pub seconds_late: Option<usize>,
    pub workflow_state: Option<String>,
    pub extra_attempts: Option<usize>,
    pub anonymous_id: Option<String>,
    pub posted_at: Option<String>,
}
