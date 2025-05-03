pub mod health;
pub mod messages;
pub mod peers;

use axum::Router;
use std::sync::Arc;
use subversive_types::state::AppState;

/// Trait for API modules that can register their routes
pub trait ApiModule {
    /// Register routes for this module
    fn register_routes() -> Router<Arc<AppState>>;
}

/// Register all API routes
pub fn register_routes(router: Router<Arc<AppState>>) -> Router<Arc<AppState>> {
    router
        .merge(health::Health::register_routes())
        .merge(peers::Peers::register_routes())
        .merge(messages::Messages::register_routes())
}
