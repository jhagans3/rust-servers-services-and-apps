use crate::state::AppState;
use actix_web::web::Data;
use std::sync::Mutex;

pub fn app_state() -> Data<AppState> {
    Data::new(AppState {
        health_check_response: "Actix is good. You've already asked me".to_string(),
        visit_count: Mutex::new(0),
    })
}
