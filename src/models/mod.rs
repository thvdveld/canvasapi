//! Models representing data structures from the Canvas API.

pub mod assignment;
pub mod course;
pub mod user;

pub mod prelude {
    pub use super::assignment::*;
    pub use super::course::*;
    pub use super::user::*;
}
