//! Model for accessing learning outcome information.

use serde::{Deserialize, Serialize};

use crate::canvas::CanvasInformation;
use crate::models::prelude::*;
use crate::parameters::*;

use crate::requests::*;

#[derive(Debug, Deserialize, Serialize)]
pub struct Outcome {
    pub id: usize,
    pub url: Option<String>,
    pub context_id: Option<usize>,
    pub context_type: Option<String>,
    pub title: Option<String>,
    pub display_name: Option<String>,
    pub description: Option<String>,
    pub vendor_guid: Option<String>,
    pub points_possible: Option<usize>,
    pub mastery_points: Option<usize>,
    pub calculation_method: Option<String>,
    pub calculation_int: Option<usize>,
    pub ratings: Option<Vec<String>>,
    pub can_edit: Option<bool>,
    pub can_unlink: Option<bool>,
    pub accessed: Option<bool>,
    pub has_updateable_rubrics: Option<bool>,
}
