//! Model for managing course information.

use serde::{Deserialize, Serialize};

use crate::canvas::CanvasInformation;
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
    api_todo! {
        /// Create a new grading standard for the course.
        add_grading_standards(self)
    }

    api_todo! {
        /// Mark this course as concluded.
        conclude(self)
    }

    api_todo! {
        /// Create a new assignment for this course.
        ///
        /// Note: the assignment is created in the active state.
        create_assignment(self)
    }

    api_todo! {
        /// Create a new assignment group for this course.
        create_assignment_group(self)
    }

    api_todo! {
        /// Create the specified overrides for each assignment.
        create_assignment_overrides(self)
    }

    api_get! {
        /// Get all courses from the current user.
        courses():
            "courses" =>
            () -> () -> [Course]
    }

    // api_get! {
    // /// Get a course with a specific Canvas ID.
    // with_id():
    // "courses/{id}" =>
    // () -> (id: usize) -> Course
    // }

    api_get! {
        /// Get all outcome links for context - BETA
        get_all_outcome_links_in_context(self):
            "courses/{id}/outcome_group_links" =>
                (id: self.id) -> () -> [String]
    }

    api_get! {
        /// Return the assignment with the given id.
        get_assignment(self):
            "courses/{id}/assignments/{assignment_id}" =>
                (id: self.id) -> (assignment_id: usize) -> Assignment
    }

    api_get! {
        /// Retrieve specified assignment group for the specified course.
        get_assignment_group(self):
            "courses/{id}/assignment_groups/{assignment_group_id}" =>
                (id: self.id) -> (assignment_group_id: usize) -> AssignmentGroup
    }

    api_get! {
        /// List assignment groups for the specified course.
        get_assignment_groups(self):
            "course/{id}/assignment_groups" =>
                (id: self.id) -> () -> [AssignmentGroup]
    }

    api_todo! {
        /// List the specified overrides in this course, providing they target
        /// sections/groups/students visible to the current user.
        get_assignment_overrides(self)
    }

    api_get! {
        /// Get all the assignments of a course.
        get_assignments(self):
            "courses/{id}/assignments" =>
            (id: self.id) -> () -> [Assignment]
    }

    api_get! {
        /// Returns a list of assignments for the given assignment group.
        get_assignments_for_group(self):
            "courses/{id}/assignment_groups/{assignment_group_id}/assignments" =>
                (id: self.id) -> (assignment_group_id: usize) -> [Assignment]
    }

    api_get! {
        /// Get all the users from the course.
        /// This includes: teachers, students, teacher assistants...
        get_users(self):
            "courses/{id}/search_users" =>
            (id: self.id) -> () -> [User]
    }

    api_get! {
        /// Get only the students from the course.
        get_students(self):
            "courses/{id}/search_users" =>
            (id: self.id) -> () -> [User]
            [EnrollmentType::Student]
    }

    api_get! {
        /// Get all the files of a course.
        get_files(self):
            "courses/{id}/files" =>
            (id: self.id) -> () -> [File]
    }
}
