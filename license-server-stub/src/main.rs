use axum::{routing::post, Json, Router};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
struct ValidatePayload {
    license_key: String,
    device_id: String,
    app_version: String,
}

#[derive(Debug, Serialize)]
struct ValidateResponse {
    valid: bool,
    plan: &'static str,
    expires_at: Option<&'static str>,
}

async fn validate(Json(payload): Json<ValidatePayload>) -> Json<ValidateResponse> {
    let _ = (payload.device_id, payload.app_version);
    let valid = payload.license_key.starts_with("MPRO-");
    Json(ValidateResponse {
        valid,
        plan: if valid { "pro" } else { "free" },
        expires_at: if valid { Some("2099-01-01T00:00:00Z") } else { None },
    })
}

#[tokio::main]
async fn main() {
    let app = Router::new().route("/validate", post(validate));
    let listener = tokio::net::TcpListener::bind("0.0.0.0:8088").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
