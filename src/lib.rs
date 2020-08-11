#![allow(dead_code, unused)]

pub mod canvas;
pub mod models;
mod parameters;
mod requests;

use canvas::Canvas;
use models::*;

pub mod prelude {
    pub use super::canvas::Canvas;
    pub use super::models::prelude::*;
}
