use serde::{Deserialize, Serialize};

use crate::canvas::Canvas;
use crate::models::prelude::*;
use crate::parameters::*;

use crate::requests::*;

#[derive(Debug, Deserialize, Serialize)]
pub struct Course {
    pub id: usize,
    pub sis_course_id: Option<String>,
    pub uuid: Option<String>,
    pub integration_id: Option<usize>,
    pub sis_import_id: Option<usize>,
    pub name: Option<String>,
    pub course_code: Option<String>,
    pub workflow_state: Option<String>,
    pub account_id: Option<usize>,
    pub root_account_id: Option<usize>,
    pub enrollment_term_id: Option<usize>,
    pub grading_standard_id: Option<usize>,
    pub grade_passback_setting: Option<String>,
    pub created_at: Option<String>,
    pub start_at: Option<String>,
    pub end_at: Option<String>,
    pub locale: Option<String>,
    // pub enrollments: Vec<>,
    pub total_students: Option<usize>,
    // pub calendar: (),
    pub default_view: Option<String>,
    pub syllabus_body: Option<String>,
    pub needs_grading_count: Option<usize>,
    // term: Option<()>,
    // course_progress: Option<()>,
    pub apply_assignment_group_weights: Option<bool>,
    pub permissions: Option<Vec<String>>,
    pub is_public: Option<bool>,
    pub is_public_to_auth_users: Option<bool>,
    pub public_syllabus: Option<bool>,
    pub public_syllabus_to_auth: Option<bool>,
    pub public_description: Option<String>,
    pub storage_quota_mb: Option<usize>,
    pub storage_quota_used_mb: Option<usize>,
    pub hide_final_grades: Option<bool>,
    pub license: Option<String>,
    pub allow_student_assignment_edits: Option<bool>,
    pub allow_wiki_comments: Option<bool>,
    pub allow_student_forum_attachments: Option<bool>,
    pub open_enrollment: Option<bool>,
    pub self_enrollment: Option<bool>,
    pub restrict_enrollments_to_course_dates: Option<bool>,
    pub course_format: Option<String>,
    pub access_restricted_by_date: Option<bool>,
    pub time_zone: Option<String>,
    pub blueprint: Option<bool>,
    // blueprint_restrictions: Option<()>,
    // blueprint_restrictions_by_object_type: Option<()>,
}

impl Course {
    /// Returns a list of active courses for the current user.
    pub fn courses() -> GetPagedObjectRequest<Course> {
        GetPagedObjectRequest::<Course>::new(format!("courses"))
    }

    /// Get a course with a specific Canvas ID.
    pub fn with_id(id: usize) -> GetObjectRequest<Course> {
        GetObjectRequest::<Course>::new(format!("courses/{}", id))
    }

    /// Get all the users from the course.
    /// This includes: teachers, students, teacher assistants...
    pub fn get_users(self) -> GetPagedObjectRequest<User> {
        GetPagedObjectRequest::<User>::new(format!("courses/{}/search_users", self.id))
    }

    /// Get only the students from the course.
    pub fn get_students(self, canvas: &Canvas) -> GetPagedObjectRequest<User> {
        let mut req =
            GetPagedObjectRequest::<User>::new(format!("courses/{}/search_users", self.id));

        req.add_parameter(EnrollmentType::Student)
    }

    /// Get all the assignments of a course.
    pub fn get_assignments(self) -> GetPagedObjectRequest<Assignment> {
        GetPagedObjectRequest::<Assignment>::new(format!("courses/{}/assignments", self.id))
    }
}
