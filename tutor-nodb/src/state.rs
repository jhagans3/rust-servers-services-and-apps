pub mod actix;
pub mod axum;
pub mod poem;

use std::sync::Mutex;
use utoipa::ToSchema;

use crate::models::Course;

#[derive(Debug, ToSchema)]
pub struct AppState {
    pub health_check_response: String,
    pub visit_count: Mutex<u32>,
    pub courses: Mutex<Vec<Course>>,
}
