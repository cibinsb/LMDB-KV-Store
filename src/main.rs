//! LMDB key/value store showing features of axum.

mod datastore;
mod handlers;
mod helper;
mod router;
mod search;

use router::app;
use tracing_subscriber::{
    layer::SubscriberExt,
    util::SubscriberInitExt
};


#[tokio::main]
async fn main() {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "LMDB_key_value_store=debug,tower_http=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();
    // Run our app with hyper
    axum::Server::bind(&"0.0.0.0:8000".parse().unwrap())
        .serve(app().into_make_service())
        .await
        .unwrap();
}
