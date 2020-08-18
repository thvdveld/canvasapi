use serde::{Deserialize, Serialize};

use crate::canvas::CanvasInformation;
use crate::models::prelude::*;
use crate::parameters::*;

use crate::requests::*;

#[derive(Debug, Deserialize, Serialize)]
pub struct Canvas;

impl Canvas {
    api_get! {
        /// Get a course with a specific id.
        get_course():
            "courses/{id}" =>
                () -> (id: usize) -> Course
    }
}
