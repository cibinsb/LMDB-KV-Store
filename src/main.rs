//! LMDB key/value store showing features of axum.

mod datastore;
mod handlers;
mod helper;
mod router;
mod search;

use std::env;
use shuttle_secrets::SecretStore;
use router::app;


#[shuttle_runtime::main]
async fn axum(#[shuttle_secrets::Secrets] secret_store: SecretStore,)
    -> shuttle_axum::ShuttleAxum {
    env::set_var("DATABASE_NAME", secret_store.get("DATABASE_NAME").unwrap());
    env::set_var("SECRET_TOKEN", secret_store.get("SECRET_TOKEN").unwrap());
    Ok(app().into())
}
