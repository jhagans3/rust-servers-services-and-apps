use ::axum::{routing::get, Router};
use std::sync::Arc;

use utoipa::{
    openapi::security::{ApiKey, ApiKeyValue, SecurityScheme},
    Modify, OpenApi,
};
use utoipa_rapidoc::RapiDoc;
use utoipa_redoc::{Redoc, Servable};
use utoipa_swagger_ui::SwaggerUi;

use crate::handlers::axum::health_check;
use crate::state::axum::app_state;

#[derive(OpenApi)]
#[openapi(
        paths(
            crate::handlers::axum::health_check,
        ),
        components(
            schemas(crate::state::AppState)
        ),
        modifiers(&SecurityAddon),
        tags(
            (name = "courses", description = "EZ tutors no db")
        )
    )]
pub struct ApiDoc;

pub struct SecurityAddon;

impl Modify for SecurityAddon {
    fn modify(&self, openapi: &mut utoipa::openapi::OpenApi) {
        if let Some(components) = openapi.components.as_mut() {
            components.add_security_scheme(
                "api_key",
                SecurityScheme::ApiKey(ApiKey::Header(ApiKeyValue::new("todo_apikey"))),
            )
        }
    }
}

pub fn axum_router() -> Router {
    let app_state = app_state();

    let shared_data = Arc::new(app_state);
    Router::new()
        .merge(SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", ApiDoc::openapi()))
        .merge(Redoc::with_url("/redoc", ApiDoc::openapi()))
        .merge(RapiDoc::new("/api-docs/openapi.json").path("/rapidoc"))
        .route("/health", get(health_check))
        .with_state(shared_data)
}
