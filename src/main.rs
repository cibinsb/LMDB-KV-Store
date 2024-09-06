 //! LMDB key/value store showing features of axum.

mod datastore;
mod handlers;
mod helper;
mod router;
mod search;

use std::env;
use router::app;
use shuttle_runtime::SecretStore;

#[shuttle_runtime::main]
async fn main(#[shuttle_runtime::Secrets] secrets: SecretStore) -> shuttle_axum::ShuttleAxum {
    env::set_var("DATABASE_NAME", secrets.get("DATABASE_NAME").unwrap());
    env::set_var("SECRET_TOKEN", secrets.get("SECRET_TOKEN").unwrap());
    Ok(app().into())
}