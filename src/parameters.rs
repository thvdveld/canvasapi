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

        impl Into<RequestParameter> for $name {
            fn into(self) -> RequestParameter {
                RequestParameter {
                    name: $name_output.into(),
                    value: match self {
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

        impl<'i> Into<RequestParameter> for $name<'i> {
            fn into(self) -> RequestParameter {
                RequestParameter {
                    name: $name_output.into(),
                    value: self.0.into(),
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
