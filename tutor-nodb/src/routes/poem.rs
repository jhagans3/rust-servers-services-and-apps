use crate::handlers::poem::EzCourseApi;
use crate::state::poem::app_state;
use crate::state::AppState;
use poem::{middleware::AddDataEndpoint, EndpointExt, Route};
use poem_openapi::OpenApiService;
use std::sync::Arc;

pub fn app() -> AddDataEndpoint<poem::Route, Arc<AppState>> {
    let course_service =
        OpenApiService::new(EzCourseApi, "Hello World", "1.0").server("http://localhost:3000/");
    let ui = course_service.swagger_ui();

    let app_state = Arc::new(app_state());

    Route::new()
        .nest("/", course_service)
        .nest("/swagger", ui)
        .data(app_state.clone())
}
