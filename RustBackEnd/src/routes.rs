use axum::{Json, extract::Form};
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct LoginRequest {
    name: String,
    email: String,
}

#[derive(Serialize)]
pub struct LoginResponse {
    message: String,
}

pub async fn login(Form(payload): Form<LoginRequest>) -> Json<LoginResponse> {
    // Add login validation or database check here
    Json(LoginResponse {
        message: format!("Welcome, {}", payload.name),
    })
}

pub async fn chat() -> Json<String> {
    Json("Chat functionality is under construction.".to_string())
}
