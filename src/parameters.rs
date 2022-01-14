//! Parameters that can be passed with a request.
//!
//! # Example
//!
//! For example, the `EnrollmentType` is added as a parameter, which will return only the students
//! for the specific course.
//! To know which parameters can be used for a request, the official Canvas LMS API documentation
//! needs to be consulted.
//!
//! ```
//! # use canvasapi::prelude::*;
//! # #[tokio::test]
//! # async fn parameter_test() {
//! #   dotenv::dotenv().ok();
//! #   let base_url = std::env::var("CANVAS_BASE_URL").unwrap();
//! #   let cvs_token = std::env::var("CANAVS_ACCESS_TOKEN").unwrap();
//! #   let canvas = CanvasInformation::new(&base_url, &cvs_token);
//! #   let course = Canvas::get_course(13369)
//! #       .unwrap()
//! #       .fetch(&canvas)
//! #       .await
//! #       .unwrap()
//! #       .inner();
//!
//! let students = course
//!     .get_users()
//!     .unwrap()
//!     .add_parameter(EnrollmentType::Student)
//!     .fetch(&canvas).await.unwrap().inner();
//! #   }
//! ```

/// Parameter that can be added to a request.
pub struct RequestParameter {
    pub name: String,
    pub value: String,
}

macro_rules! api_parameter {
    (
        $(#[$outer:meta])*
        $name:ident => $name_output:expr,
        $(
            $(#[$option_outer:meta])*
            $option:ident => $option_output:expr ,
        )* $(,)?
    ) => {
        $(#[$outer])*
        pub enum $name {
            $(
                $(#[$option_outer])*
                $option,
            )*
        }

        impl From<$name> for RequestParameter {
            fn from(value: $name) -> Self {
                RequestParameter {
                    name: $name_output.into(),
                    value: match value {
                        $(<$name>::$option => $option_output,)*
                    }.into(),
                }
            }
        }
    };

    (
        $(#[$outer:meta])*
        $name:ident => $name_output:expr
    ) => {
        $(#[$outer])*
        pub struct $name<'i>(pub &'i str);

        impl<'i> From<$name<'i>> for RequestParameter {
            fn from(value: $name) -> Self {
                RequestParameter {
                    name: $name_output.into(),
                    value: value.0.into(),
                }
            }
        }
    };
}

api_parameter! {
    /// If this parameter is given and it corresponds to a user in the course, the page parameter
    /// will be ignored and the page containing the specified user will be returned instead.
    UserId => "user_id"
}

api_parameter! {
    /// Parameter to specify the enrollment type of a user.
    EnrollmentType => "enrollment_type[]",
    Teacher => "teacher",
    Student => "student",
    StudentView => "student_view",
    TA => "ta",
    Observer => "observer",
    Designer => "designer",
}

api_parameter! {
    /// Parameter to specify on wich field the output is sorted.
    SortOn => "sort",
    Username => "username",
    LastLogin => "last_login",
    Email => "email",
    SisId => "sis_id",
}

api_parameter! {
    EnrollmentState => "enrollment_state[]",
    Active => "active",
    Invited => "invited",
    Rejected => "rejected",
    Completed => "completed",
    Inactive => "inactive",
    InvitedOrPending => "invited_or_pending",
}

api_parameter! {
    Include => "include[]",
    Enrollments => "enrollments",
    Locked => "locked",
    AvatarUrl => "avatar_url",
    TestStudent => "test_student",
    Bio => "bio",
    CustomLinks => "custom_links",
    CurrentGradingPeriodScores => "current_grading_period_scores",
    Uuid => "uuid",
    NeedsGradingCount => "needs_grading_count",
    SyllabusBody => "syllabus_body",
    PublicDescription => "public_description",
    TotalScores => "total_scores",
    /// The information for the enrollment term for each course is returned.
    Term => "term",
    Account => "account",
    CourseProgress => "course_progress",
    Sections => "sections",
    StorageQuotaUsedMb => "storage_quota_used_mb",
    TotalStudents => "total_students",
    PassbackStatus => "passback_status",
    Favorites => "favorites",
    Teachers => "teachers",
    ObservedUsers => "observed_users",
    CourseImage => "course_image",
    Concluded => "concluded",
}
