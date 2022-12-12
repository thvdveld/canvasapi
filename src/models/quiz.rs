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
    pub unlock_at: Option<String>,
    pub asset_string: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Quiz {
    pub id: Option<i64>,
    pub title: Option<String>,
    pub html_url: Option<String>,
    pub mobile_url: Option<String>,
    pub preview_url: Option<String>,
    pub description: Option<String>,
    pub quiz_type: Option<String>,
    pub assignment_group_id: Option<i64>,
    pub time_limit: Option<i64>,
    pub shuffle_answers: Option<bool>,
    pub hide_results: Option<String>,
    pub show_correct_answers: Option<bool>,
    pub show_correct_answers_last_attempt: Option<bool>,
    pub show_correct_answers_at: Option<String>,
    pub hide_correct_answers_at: Option<String>,
    pub one_time_results: Option<bool>,
    pub scoring_policy: Option<String>,
    pub allowed_attempts: Option<i64>,
    pub one_question_at_a_time: Option<bool>,
    pub question_count: Option<i64>,
    pub points_possible: Option<i64>,
    pub cant_go_back: Option<bool>,
    pub access_code: Option<String>,
    pub ip_filter: Option<String>,
    pub due_at: Option<String>,
    pub lock_at: Option<String>,
    pub unlock_at: Option<String>,
    pub published: Option<bool>,
    pub unpublishable: Option<bool>,
    pub locked_for_user: Option<bool>,
    pub lock_info: Option<LockInfo>,
    pub lock_explanation: Option<String>,
    pub speedgrader_url: Option<String>,
    pub quiz_extensions_url: Option<String>,
    pub permissions: Option<Value>,
    pub all_dates: Option<AllDates>,
    pub version_number: Option<i64>,
    pub question_types: Option<Vec<String>>,
    pub anonymous_submissions: Option<bool>,
}