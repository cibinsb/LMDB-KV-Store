
use axum::{
    error_handling::HandleErrorLayer,
    extract::DefaultBodyLimit,
    handler::Handler,
    routing::{get, post_service},
    Router,
};
use std::{
    sync::{Arc, RwLock},
    time::Duration,
};
use tower::ServiceBuilder;
use tower_http::compression::CompressionLayer;
use tower_http::limit::RequestBodyLimitLayer;
use tower_http::trace::TraceLayer;
use crate::datastore::LMDBStore;
use crate::handlers::*;
use crate::helper::handle_error;

pub type SharedState = Arc<RwLock<LMDBStore>>;

pub fn app() -> Router {
    let shared_state = SharedState::default();
    // Build our application by composing routes
    let app: Router = Router::new()
        .route(
            "/value",
            // But don't compress `kv_set`
            post_service(
                kv_set_no_key
                    .layer((
                        DefaultBodyLimit::disable(),
                        RequestBodyLimitLayer::new(1024 * 5_000 /* ~5mb */),
                    ))
                    .with_state(Arc::clone(&shared_state)),
            ),
        )
        .route(
            "/:key",
            // Add compression to `kv_get`
            get(kv_get.layer(CompressionLayer::new()))
                // But don't compress `kv_set`
                .post_service(
                    kv_set
                        .layer((
                            DefaultBodyLimit::disable(),
                            RequestBodyLimitLayer::new(1024 * 5_000 /* ~5mb */),
                        ))
                        .with_state(Arc::clone(&shared_state)),
                ),
        )
        .route("/keys", get(list_keys))
        .route("/search", get(kv_search))
        // Nest our admin routes under `/admin`
        .nest("/admin", admin_routes())
        // Add middleware to all routes
        .layer(
            ServiceBuilder::new()
                // Handle errors from middleware
                .layer(HandleErrorLayer::new(handle_error))
                .load_shed()
                .concurrency_limit(2048)
                .timeout(Duration::from_secs(10))
                .layer(TraceLayer::new_for_http()),
        )
        .with_state(Arc::clone(&shared_state));
    app
}