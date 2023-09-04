use crate::state::AppState;
use actix_web::web::Data;
use sqlx::{Pool, Postgres};
use std::sync::Mutex;

pub fn app_state(db_pool: Pool<Postgres>) -> Data<AppState> {
    Data::new(AppState {
        health_check_response: "Actix is good. You've already asked me".to_string(),
        visit_count: Mutex::new(0),
        db: db_pool,
    })
}
