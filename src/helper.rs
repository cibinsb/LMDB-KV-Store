use std::borrow::Cow;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use base64::URL_SAFE;
use rand::Rng;
use tower::BoxError;

pub async fn handle_error(error: BoxError) -> impl IntoResponse {
    if error.is::<tower::timeout::error::Elapsed>() {
        return (StatusCode::REQUEST_TIMEOUT, Cow::from("request timed out"));
    }

    if error.is::<tower::load_shed::error::Overloaded>() {
        return (
            StatusCode::SERVICE_UNAVAILABLE,
            Cow::from("service is overloaded, try again later"),
        );
    }
    (
        StatusCode::INTERNAL_SERVER_ERROR,
        Cow::from(format!("Unhandled internal error: {}", error)),
    )
}

pub fn generate() -> String {
    let mut rng = rand::thread_rng();
    let mut buffer: [u8; 20] = [0; 20];
    rng.fill(&mut buffer);
    base64::encode_config(&buffer, URL_SAFE)
}
