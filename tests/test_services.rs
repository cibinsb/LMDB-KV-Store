use std::env;
use axum::{body::Body, http::{Request, StatusCode}, http};
use http::header::{self, HeaderValue};
use serde_json::{json, Value, from_slice, from_str};
use tower::ServiceExt;
use lmdb_kv_store::router::app; // for `oneshot` and `ready`
use axum_test::TestServer;

fn start_server() -> TestServer {
    // set env
    env::set_var("DATABASE_NAME", "test.mdb");
    env::set_var("SECRET_TOKEN", "secret-token");
    // Build an application with a route.
    let app = app()
      .into_make_service();

    // Run the server on a random address.
    let server = TestServer::new(app).unwrap();
    server
}

#[tokio::test]
async fn count_keys() {
    let app = app();
    let response = app
        .oneshot(Request::builder().uri("/admin/keys/count")
            .header("Authorization", "Bearer secret-token")
            .body(Body::empty()).unwrap())
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    // let body = hyper::body::to_bytes(response.into_body()).await.unwrap();
    // let body: Value = from_slice(&body).unwrap();
    // assert_eq!(body, json!({ "count": 0 }));
}

#[tokio::test]
async fn check_keys() {

    let server = start_server();
    // Get the request.
    let response = server
      .get("/keys")
      .await;
    // let body: Value = from_str(&response.text()).unwrap();
    // assert_eq!(body, json!({ "data": {} }));
    assert_eq!(response.status_code(), StatusCode::OK);
}

#[tokio::test]
async fn post_keys() {
    let server = start_server();
    // Get the request.
    let response = server
      .post("/key1").json("fake test value")
      .await;
    assert_eq!(response.status_code(), StatusCode::OK);
}

#[tokio::test]
async fn post_value() {
    let server = start_server();
    // Get the request.
    let response = server
      .post("/value").json("fake test value")
      .await;
    assert_eq!(response.status_code(), StatusCode::OK);
}

#[tokio::test]
async fn delete_key() {
    let server = start_server();
    let response = server
      .post("/key2").json("fake test value")
      .await;
    assert_eq!(response.status_code(), StatusCode::OK);
    // Get the request.
    let response = server
      .delete("/admin/key/key2")
        .add_header(header::AUTHORIZATION,
                    HeaderValue::from_str("Bearer secret-token").unwrap())
      .await;
    assert_eq!(response.status_code(), StatusCode::OK);
}

#[tokio::test]
async fn delete_keys() {
    let server = start_server();
    let response = server
      .post("/key2").json("fake test value")
      .await;
    assert_eq!(response.status_code(), StatusCode::OK);
    // Get the request.
    let response = server
      .delete("/admin/keys")
        .add_header(header::AUTHORIZATION,
                    HeaderValue::from_str("Bearer secret-token").unwrap())
      .await;
    assert_eq!(response.status_code(), StatusCode::OK);
}