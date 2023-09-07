use chrono::NaiveDateTime;
use poem_openapi::Object;

use serde::{Deserialize, Serialize};

#[derive(Object, Clone, Serialize, Deserialize)]
pub struct Course {
    pub tutor_id: i32,
    pub course_id: Option<i32>,
    pub course_name: String,
    pub posted_time: Option<NaiveDateTime>,
}
