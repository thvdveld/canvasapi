//! Models for accessing information about assignments.

use anyhow::anyhow;
use serde::{Deserialize, Serialize};
use serde_json::{Value};

use crate::canvas::CanvasInformation;
use crate::models::prelude::*;
use crate::parameters::*;
use crate::requests::*;

#[derive(Debug, Deserialize, Serialize)]
pub struct Assignment {
    pub id: Option<usize>,
    pub name: Option<String>,
    pub description: Option<String>,
    pub created_at: Option<String>,
    pub updated_at: Option<String>,
    pub due_at: Option<String>,
    pub lock_at: Option<String>,
    pub unlock_at: Option<String>,
    pub has_overrides: Option<bool>,
    pub all_dates: Option<Vec<AllDates>>,
    pub course_id: Option<usize>,
    pub html_url: Option<String>,
    pub submission_download_url: Option<String>,
    pub assignment_group_id: Option<usize>,
    pub due_date_required: Option<bool>,
    pub allowed_extensions: Option<Vec<String>>,
    pub max_name_length: Option<usize>,
    pub turnitin_enabled: Option<bool>,
    pub vericite_enabled: Option<bool>,
    // pub turnitin_settings: usize,
    pub grade_group_studens_individually: Option<bool>,
    // pub external_tool_tag_attributes: usize,
    pub peer_reviews: Option<bool>,
    pub automatic_peer_reviews: Option<bool>,
    pub peer_review_count: Option<usize>,
    pub peer_reviews_assign_at: Option<String>,
    pub intra_group_peer_review: Option<bool>,
    pub group_category_id: Option<usize>,
    pub needs_grading_count: Option<usize>,
    // pub needs_grading_count_by_section: usize,
    pub position: Option<usize>,
    pub post_to_sis: Option<bool>,
    pub integration_id: Option<String>,
    // pub integration_data: usize,
    pub points_possible: Option<f64>,
    pub submission_types: Option<Vec<String>>,
    pub has_submitted_submissions: Option<bool>,
    pub grading_type: Option<String>,
    pub rading_standard_id: Option<String>,
    pub published: Option<bool>,
    pub unpublishable: Option<bool>,
    pub only_visible_to_overrides: Option<bool>,
    pub locked_for_user: Option<bool>,
    pub lock_info: Option<LockInfo>,
    pub lock_explanation: Option<String>,
    pub quiz_id: Option<usize>,
    pub anonymous_submissions: Option<bool>,
    // pub discussion_topic: usize,
    pub freeze_on_copy: Option<bool>,
    pub frozen: Option<bool>,
    pub frozen_attributes: Option<Vec<String>>,
    // pub submission: usize,
    pub use_rubric_for_grading: Option<bool>,
    // pub rubric_settings: usize,
    // pub rubric: usize,
    pub assignment_visibility: Option<Vec<usize>>,
    // pub overrides:usize,
    pub omit_from_final_grade: Option<bool>,
    pub moderated_grading: Option<bool>,
    pub grader_count: Option<usize>,
    pub final_grader_id: Option<usize>,
    pub grader_comments_visible_to_graders: Option<bool>,
    pub graders_anonymous_to_grader: Option<bool>,
    pub grader_names_visible_to_final_grader: Option<bool>,
    pub anonymous_grading: Option<bool>,
    pub allowed_attemts: Option<usize>,
    pub post_manually: Option<bool>,
}

impl Assignment {
    api_get! {
        /// List students eligible to submit this assignment.
        get_gradeable_students(self):
            "courses/{course_id}/assignments/{id}/gradeable_students" =>
            (
                course_id: self.course_id.ok_or_else(|| anyhow!("Field `course_id` missing"))?,
                id: self.id.ok_or_else(|| anyhow!("Field `id` missing"))?,
            ) -> () -> [UserDisplay]
    }

    api_get! {
        /// Get a single submission, based on user id.
        get_submission(self):
             "courses/{course_id}/assignments/{id}/submissions/{user_id}" =>
            (
                course_id: self.course_id.ok_or_else(|| anyhow!("Field `course_id` missing"))?,
                id: self.id.ok_or_else(|| anyhow!("Field `id` missing"))?,
            ) -> (user_id: usize) -> Submission
    }

    api_get! {
        /// Get all existing submissions for this assignment.
        get_submissions(self):
            "courses/{course_id}/assignments/{id}/submissions" =>
            (
                course_id: self.course_id.ok_or_else(|| anyhow!("Field `course_id` missing"))?,
                id: self.id.ok_or_else(|| anyhow!("Field `id` missing"))?,
            ) -> () -> [Submission]
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct AssignmentGroup {
    pub id: usize,
    pub name: Option<String>,
    pub position: Option<usize>,
    pub group_weight: Option<usize>,
    pub sis_source_id: Option<String>,
    // pub integration_data: usize,
    pub assignments: Option<Vec<usize>>,
    // pub rules: usize,
}
