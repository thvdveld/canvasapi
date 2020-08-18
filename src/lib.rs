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
