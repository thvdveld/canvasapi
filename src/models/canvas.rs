//! Model for accessing root level API requests.

use serde::{Deserialize, Serialize};

use crate::canvas::CanvasInformation;
use crate::models::prelude::*;
use crate::parameters::*;

use crate::requests::*;

/// Model for accessing root level API requests.
#[derive(Debug, Deserialize, Serialize)]
pub struct Canvas;

impl Canvas {
    api_todo! {
        /// Returns any currently running conversation batches for the current user.
        /// Conversation batches are created when a bulk private message is sent asychronously.
        conversations_get_running_batches()
    }

    api_get! {
        /// Get the number of unread conversations for the current user.
        conversations_unread_count():
            "conversations/unread_count" =>
                () -> () -> { "unread_count": usize }
    }

    api_todo! {
        /// Retrieve information on an individual account.
        get_account()
    }

    api_todo! {
        /// List accounts that the current user can view or manage.
        ///
        /// Typically students and teachers will get an empty list in response.
        /// Only account admins can view the accounts that they are in.
        get_accounts()
    }

    api_todo! {
        /// Return a summary of the current user's global activity stream.
        get_activity_stream_summary(self)
    }

    api_todo! {
        /// List announcements.
        get_announcements()
    }

    api_todo! {
        /// Return single Appointment Group by id
        get_appointment_group()
    }

    api_todo! {
        /// List appointment groups.
        get_appointment_groups()
    }

    api_todo! {
        /// Get accoutn brand variables.
        get_brand_variables()
    }

    api_todo! {
        /// Return single Calendar Event by ID.
        get_calendar_event()
    }

    api_todo! {
        /// List calendar events.
        get_calendar_events()
    }

    api_todo! {
        /// Retrieve a list of messages sent to a user.
        get_comm_messages()
    }

    api_todo! {
        /// Return a single conversation.
        get_conversation()
    }

    api_todo! {
        /// Get conversations;
        get_conversations()
    }

    api_get! {
        /// Get a course with a specific id.
        get_course():
            "courses/{id}" =>
                () -> (id: usize) -> Course
    }

    api_todo! {
        /// List accounts that the current user can view through their admin course enrollments
        /// (Teacher, TA or designer enrollments).
        ///
        /// Only returns ID, name, workflow_state, root_account_id and parent_account_id.
        get_course_accounts()
    }

    api_todo! {
        /// Return the nickname for the given course.
        get_course_nickname()
    }

    api_todo! {
        /// Return all course nicknames set by the current account.
        get_course_nicknames()
    }

    api_todo! {
        /// Return a list of active courses for the current user.
        get_courses()
    }

    api_todo! {
        /// Return a list of epub exports for the associated course.
        get_epub_exports()
    }

    api_todo! {
        /// Return the standard attachment json object for a file.
        get_file()
    }

    api_todo! {
        /// Return the details for a folder.
        get_folder()
    }

    api_todo! {
        /// Returns the data for a signle group.
        get_group()
    }

    api_todo! {
        /// Get a single group category.
        get_group_category()
    }

    api_todo! {
        /// List student group participants in the appointment group.
        get_group_participants()
    }

    api_todo! {
        /// Returns the details of the outcome with the given ID.
        get_outcome()
    }

    api_todo! {
        /// Returns the details of the outcome group with the given ID.
        get_outcome_group()
    }

    api_todo! {
        /// Retrieve a planner note for the current user.
        get_planner_note()
    }

    api_todo! {
        /// Retrieve the list of planner notes.
        get_planner_notes()
    }

    api_todo! {
        /// Retrieve a planner override for the current user.
        get_planner_overrides()
    }

    api_todo! {
        /// Get a single poll, based on the poll ID.
        get_poll()
    }

    api_todo! {
        /// Get a list of polls for the current user.
        get_polls()
    }

    api_todo! {
        /// Redirect to root outcome group for context.
        get_root_outcome_group()
    }

    api_todo! {
        /// Get details about a specific section.
        get_section()
    }

    api_get! {
        /// Return the current user's list of todo items, as seen on the user dashboard.
        get_todo_items():
            "users/self/todo" =>
                () -> () -> [std::collections::HashMap<String,serde_json::Value>]
            features = [( name = "devel", reason = "The return type is not correct yet.")]
    }

    api_todo! {
        /// Return the current user's upcomming events, i.e. the same things shown in the dashboard
        /// 'Comming Up' sidebar.
        get_upcomming_events()
    }

    api_todo! {
        /// Retrieve a user by their ID.
        get_user()
    }

    api_todo! {
        /// List user participants in this appointment group.
        get_user_participants()
    }

    api_todo! {
        /// Returns a list of up to 5 matching account domains.
        /// Partial matches on name and domain are supported.
        searh_accounts()
    }

    /// Returns a list of courses that match the search criteria.
    pub fn search_course(
        search: String,
        public_only: bool,
        open_enrollment_only: bool,
    ) -> anyhow::Result<GetPagedObjectRequest<serde_json::Value>> {
        let mut request = GetPagedObjectRequest::<_>::new("search/all_courses/".to_string())
            .add_parameter(RequestParameter {
                name: "search".to_string(),
                value: search,
            });

        if public_only {
            request = request.add_parameter(RequestParameter {
                name: "public_only".into(),
                value: public_only.to_string(),
            });
        }

        if open_enrollment_only {
            request = request.add_parameter(RequestParameter {
                name: "open_enrollment_only".into(),
                value: open_enrollment_only.to_string(),
            });
        }

        Ok(request)
    }

    api_todo! {
        /// Find valid recipients (user, courses and groups) that the current user can send
        /// messages to.
        /// Returns a list of mixed data types.
        search_recipients()
    }
}
