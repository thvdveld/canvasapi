//! This is the (unofficial) Rust version of the library for accessing the Canvas LMS API.
//! The documentation of the official API can be found
//! [here](https://canvas.instructure.com/doc/api/).
//!
//! This client that is used for making the calls is [reqwest](https://crates.io/crates/reqwest).
//! The API calls are by default asynchronous.
//! The [Tokio](https://crates.io/crates/tokio) runtime is used for the tests.
//!
//! # Feature flags
//!
//! - `blocking`: enables blocking requests.
//! - `devel`: enables functions that are still in development.
//!
//! # Quickstart
//!
//! First, the information about the Canvas LMS server needs to be stored in a `CanvasInformation`
//! struct.
//! Using this information, the correct request URL's are created and, with a valid API token,
//! executed.
//!
//! ```
//! # use canvasapi::prelude::*;
//! # tokio_test::block_on(async {
//! # dotenv::dotenv().ok();
//! let base_url = std::env::var("CANVAS_BASE_URL").unwrap();
//! let canvas_token = std::env::var("CANVAS_ACCESS_TOKEN").unwrap();
//!
//! let canvas = CanvasInformation::new(&base_url, &canvas_token);
//! let course = Canvas::get_course(13369).unwrap().fetch(&canvas).await.unwrap().inner();
//! # });
//! ```
//!
//! # Contributing
//!
//! This crate only supports GET requests, and not all of them are implemented.
//! Only the ones I use are implemented.
//! Feel free to add more.
//!
//! To add support for new request, follow these steps:
//! 1. Canvas will return data in the form of JSON. This data needs to be deserialized into
//!    a struct. Define this struct in this library, under `models`, when it is not yet defined.
//!    This information can be retrieved using the official Canvas API.
//! 2. API request functions can are added in the implementation of the structs. These requests can
//!    be methods or functions. To define a new request, use the `api_get!` macro. The following
//!    example shows the usage of the macro.
//!
//! ```ignore
//! impl Course {
//!     api_get! {
//!         /// Get all courses from the current user.
//!         courses():
//!             "courses" =>
//!             () -> () -> [Course]
//!     }
//!
//!     api_get! {
//!         /// Return the assignment with the given id.
//!         get_assignment(self):
//!             "courses/{id}/assignments/{assignment_id}" =>
//!                 (id: self.id) -> (assignment_id: usize) -> Assignment
//!     }
//!
//!     api_get! {
//!         /// Get only the students from the course.
//!         get_students(self):
//!             "courses/{id}/search_users" =>
//!                 (id: self.id) -> () -> [User]
//!                 [EnrollmentType::Student]
//!     }
//!
//!     api_get! {
//!         /// This is still in development.
//!         function_in_developmen():
//!             "test" =>
//!                 () -> () -> Test
//!             features = [(name = "devel", reason = "Function is not ready yet.")]
//!     }
//!
//!     api_todo! {
//!         /// This function still needs to be fully implemented.
//!         function_that_is_todo()
//!     }
//! }
//! ```
//!
//! First, you write the (optional) documentation string, for use in this generated
//! documentation, followed by the function/method name.
//! If it is a method, the `self` keyword is used between the parentheses.
//! Next, you write the request url, without the base url and without `/api/v1/`.
//! The full request is generated using the `CanvasInformation` struct.
//! The request url can contain named parameters.
//! The `self` arguments are defined in the first group and the function arguments are
//! passed in the second group.
//! Finaly, the return type of the request is defined by passing the returned struct.
//! Square brackets are used when a `Vec` is returned by the API.
//! Optionaly, requests parameters can be added.

#![allow(dead_code, unused)]

#[macro_use]
pub mod parameters;
#[macro_use]
mod requests;

pub mod canvas;
pub mod models;

pub mod prelude {
    pub use super::canvas::CanvasInformation;
    pub use super::models::prelude::*;
    pub use super::parameters::*;
}
