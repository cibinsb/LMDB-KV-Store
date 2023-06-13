//! LMDB key/value store showing features of axum.

mod datastore;
mod handlers;
mod helper;

use datastore::LMDBStore;
use handlers::*;
use helper::{generate,handle_error};

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
use tower_http::{
    compression::CompressionLayer, limit::RequestBodyLimitLayer, trace::TraceLayer,
};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
type SharedState = Arc<RwLock<LMDBStore>>;




#[tokio::main]
async fn main() {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "LMDB_key_value_store=debug,tower_http=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    let shared_state = SharedState::default();

    // Build our application by composing routes
    let app = Router::new()
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

    // Run our app with hyper
    axum::Server::bind(&"0.0.0.0:8000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}