use crate::types::state::AppState;
use axum::Router;
use std::{path::PathBuf, sync::Arc};
use tower_http::{
    classify::{ServerErrorsAsFailures, SharedClassifier},
    cors::{Any, CorsLayer},
    services::ServeDir,
    trace::{DefaultMakeSpan, DefaultOnRequest, DefaultOnResponse, TraceLayer},
};
use tracing::Level;

/// Server configuration and middleware components
pub(crate) struct ServerComponents {
    cors_layer: CorsLayer,
    trace_layer: TraceLayer<SharedClassifier<ServerErrorsAsFailures>>,
    static_files_service: ServeDir,
}

impl ServerComponents {
    /// Initialize server components
    pub fn initialize() -> Self {
        // Set up CORS
        let cors_layer = CorsLayer::new().allow_origin(Any).allow_methods(Any);

        // Set up static file serving from public directory
        let public_dir = PathBuf::from("public");
        let static_files_service = ServeDir::new(public_dir);

        // Set up logging middleware with default span maker
        let trace_layer = TraceLayer::new_for_http()
            .make_span_with(DefaultMakeSpan::new().level(Level::INFO))
            .on_request(DefaultOnRequest::new().level(Level::INFO))
            .on_response(DefaultOnResponse::new().level(Level::INFO));

        Self {
            cors_layer,
            trace_layer,
            static_files_service,
        }
    }

    /// Configure the router with middleware and services
    pub fn configure_router(
        self,
        app_state: Arc<AppState>,
        router: Router<Arc<AppState>>,
    ) -> Router<Arc<AppState>> {
        router
            .layer(self.cors_layer)
            .layer(self.trace_layer)
            .fallback_service(self.static_files_service)
            .with_state(app_state)
    }
}
