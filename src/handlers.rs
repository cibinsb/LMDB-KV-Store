use std::boxed::Box;
use std::env;
use std::collections::HashMap;
use crate::datastore::KV;
use crate::router::SharedState;
use crate::helper::generate;
use crate::search::{index,search,Params};
use serde_json::json;
use tower_http::validate_request::ValidateRequestHeaderLayer;

use axum::{
    body::Bytes,
    extract::{Path, State, Json, Query},
    http::StatusCode,
    routing::{get, delete},
    response::{IntoResponse},
    Router,
};

pub async fn kv_search(
    Query(params): Query<Params>,
    State(state): State<SharedState>,
) -> impl IntoResponse {
    let datastore = &state.read().unwrap();
    Json(json!({"data": search(params.query, datastore)}))
}

pub async fn kv_get(
    Path(key): Path<String>,
    State(state): State<SharedState>,
) -> Result<String, StatusCode> {
    let datastore = &state.read().unwrap();
    let rtxn = datastore.env.read_txn().unwrap();
    if let Some(kv) = datastore.db.get(&rtxn, &key).unwrap() {
        Ok(kv.log)
    } else {
        Err(StatusCode::NOT_FOUND)
    }
}

pub async fn kv_set_no_key(State(state): State<SharedState>, bytes: Bytes) {
    let payload_str = String::from_utf8_lossy(&bytes);
    let datastore = &state.write().unwrap();
    let mut wtxn = datastore.env.write_txn().unwrap();
    let kv_log = KV {
        log: (payload_str).parse().unwrap(),
    };
    let key = generate();
    datastore
        .db
        .put(&mut wtxn, &key, &kv_log)
        .unwrap();
    wtxn.commit().unwrap();
    index(kv_log, datastore, key);
}

pub async fn kv_set(Path(key): Path<String>, State(state): State<SharedState>, bytes: Bytes) {
    let payload_str = String::from_utf8_lossy(&bytes);
    let datastore = &state.write().unwrap();
    let mut wtxn = datastore.env.write_txn().unwrap();
    let kv_log = KV {
        log: (&payload_str).parse().unwrap(),
    };
    datastore.db.put(&mut wtxn, &key, &kv_log).unwrap();
    wtxn.commit().unwrap();
    index(kv_log, datastore, key);
}

pub async fn list_keys(State(state): State<SharedState>) -> impl IntoResponse {
    let datastore = &state.read().unwrap();
    let rtxn = datastore.env.read_txn().unwrap();
    let mut result: Box<HashMap<String, String>> = Box::default();  // Store result on the heap
    let mut iter = datastore.db.iter(&rtxn).unwrap();
    while let Some(Ok((key, value))) = iter.next() {
        result.insert(key.to_string(), value.log);
    }
    Json(json!({"data": *result}))  // Dereference result when using it
}

pub fn admin_routes() -> Router<SharedState> {
    let secret_token = env::var("SECRET_TOKEN").expect("Missing SECRET_TOKEN!");
    async fn delete_all_keys(State(state): State<SharedState>) {
        let datastore = &state.write().unwrap();
        let mut wtxn = datastore.env.write_txn().unwrap();
        datastore.db.clear(&mut wtxn).unwrap();
        wtxn.commit().unwrap();
    }

    async fn remove_key(Path(key): Path<String>, State(state): State<SharedState>) {
        let datastore = &state.write().unwrap();
        let mut wtxn = datastore.env.write_txn().unwrap();
        datastore.db.delete(&mut wtxn, &key).unwrap();
        wtxn.commit().unwrap();
    }
    async fn count_keys(State(state): State<SharedState>) -> impl IntoResponse {
        let datastore = &state.read().unwrap();
        let rtxn = datastore.env.read_txn().unwrap();
        Json(json!({"count":datastore.db.len(&rtxn).unwrap()}))
    }

    Router::new()
        .route("/keys/count", get(count_keys))
        .route("/keys", delete(delete_all_keys))
        .route("/key/:key", delete(remove_key))
        // Require bearer auth for all admin routes
        .layer(ValidateRequestHeaderLayer::bearer(&secret_token))
}
