use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AllDates {
    pub due_at: Option<String>,
    pub id: Option<i64>,
    pub lock_at: Option<String>,
    pub set_id: Option<i64>,
    pub set_type: Option<String>,
    pub title: Option<String>,
    pub unlock_at: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LockInfo {
    pub unlock_at: String,
    pub asset_string: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Quiz {
    pub id: i64,
    pub title: String,
    pub html_url: String,
    pub mobile_url: String,
    pub preview_url: String,
    pub description: String,
    pub quiz_type: String,
    pub assignment_group_id: i64,
    pub time_limit: i64,
    pub shuffle_answers: bool,
    pub hide_results: String,
    pub show_correct_answers: bool,
    pub show_correct_answers_last_attempt: bool,
    pub show_correct_answers_at: String,
    pub hide_correct_answers_at: String,
    pub one_time_results: bool,
    pub scoring_policy: String,
    pub allowed_attempts: i64,
    pub one_question_at_a_time: bool,
    pub question_count: i64,
    pub points_possible: i64,
    pub cant_go_back: bool,
    pub access_code: String,
    pub ip_filter: String,
    pub due_at: String,
    pub lock_at: Option<String>,
    pub unlock_at: String,
    pub published: bool,
    pub unpublishable: bool,
    pub locked_for_user: bool,
    pub lock_info: Option<LockInfo>,
    pub lock_explanation: String,
    pub speedgrader_url: String,
    pub quiz_extensions_url: String,
    pub permissions: Option<Value>,
    pub all_dates: Option<AllDates>,
    pub version_number: i64,
    pub question_types: Vec<String>,
    pub anonymous_submissions: bool,
}
