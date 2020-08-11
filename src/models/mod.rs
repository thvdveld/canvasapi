//! Models representing data structures from the Canvas API.

pub mod course;
pub mod user;

pub mod prelude {
    pub use super::course::*;
    pub use super::user::*;
}
