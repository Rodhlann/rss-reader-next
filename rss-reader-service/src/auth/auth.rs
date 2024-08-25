use axum::extract::{Request, State};
use axum::response::IntoResponse;
use axum::middleware::Next;
use reqwest::{Client, StatusCode};
use serde::Deserialize;
use shuttle_runtime::SecretStore;

use crate::AppState;

#[derive(Debug, Deserialize)]
struct AuthenticatedUser {
  id: i32
}

#[derive(Debug)]
struct AuthError(String);

impl AuthError {
  fn new(msg: &str) -> Self {
    AuthError(msg.to_string())
  }
}

impl IntoResponse for AuthError {
  fn into_response(self) -> axum::response::Response {
      (StatusCode::UNAUTHORIZED, format!("Unauthorized: {}", self.0)).into_response()
  }
}

pub async fn auth_middleware (
  State(state): State<AppState>,
  req: Request,
  next: Next,
) -> Result<impl IntoResponse, impl IntoResponse> {
  let (parts, body) = req.into_parts();
  let headers = parts.headers.clone();

  let bearer_token = headers.get("Authorization")
    .and_then(|value| value.to_str().ok())
    .ok_or_else(|| AuthError::new("Missing or invalid Authorization header"))?;

  let http_client = Client::new();
  let response = http_client
    .get("https://api.github.com/user")
    .header("Authorization", bearer_token)
    .header("User-Agent", "rss-reader-service")
    .send()
    .await
    .map_err(|e| AuthError::new(&format!("Network error: {}", e)))?;

  let response_text = response.text().await.map_err(|e| AuthError::new(&format!("Error reading response body: {}", e)))?;

  let user_info: AuthenticatedUser = serde_json::from_str(&response_text)
    .map_err(|e| AuthError::new(&format!("Error decoding JSON: {}", e)))?;

  let admin_user_id = SecretStore::get(&state.secrets, "GITHUB_USER_ID")
    .ok_or_else(|| AuthError::new("Missing expected ENV_VAR: GITHUB_USER_ID"))?;
  
  if user_info.id.to_string() != admin_user_id {
    return Err(AuthError::new("Unauthorized User Action"));
  }

  let req = Request::from_parts(parts, body);
  let response = next.run(req).await;

  Ok(response)
}