use axum::extract::{Request, State};
use axum::response::IntoResponse;
use axum::middleware::Next;
use chrono::{Duration, NaiveDateTime, Utc};
use reqwest::{Client, StatusCode};
use serde::Deserialize;
use shuttle_runtime::SecretStore;

use crate::AppState;

#[derive(Debug, Deserialize)]
struct AuthenticatedUser {
  id: i32
}

#[derive(Debug, Deserialize)]
struct GitHubTokenCheck {
  created_at: String,
  user: AuthenticatedUser
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

async fn invalidate_expired_token(secrets: &SecretStore, access_token: &str) -> Result<(), AuthError> {
  let github_client_id = SecretStore::get(secrets, "GITHUB_CLIENT_ID")
    .ok_or_else(|| AuthError::new("Missing expected ENV_VAR: GITHUB_CLIENT_ID"))?;
  let github_client_secret = SecretStore::get(secrets, "GITHUB_CLIENT_SECRET")
    .ok_or_else(|| AuthError::new("Missing expected ENV_VAR: GITHUB_CLIENT_SECRET"))?;

  let http_client = Client::new();
  let response = http_client
    .delete(format!("https://api.github.com/applications/{github_client_id}/token"))
    .header("Accept", "application/vnd.github+json")
    .header("content-type", "application/json")
    .header("User-Agent", "rss-reader-service")
    .basic_auth(github_client_id, Some(github_client_secret))
    .body(format!("{{\"access_token\":\"{access_token}\"}}"))
    .send()
    .await
    .map_err(|e| AuthError::new(&format!("Network error: {}", e)))?;

  let status_code = &response.status();
  if status_code != &StatusCode::OK {
    let response_text: String = response.text().await.map_err(|e| AuthError::new(&format!("Error reading response body: {}", e)))?;
    return Err(AuthError::new(&format!("Unable to invalidate expired token! {}", response_text)));
  }
  
  Ok(())
}

async fn fetch_github_user_id(secrets: &SecretStore, access_token: &str) -> Result<String, AuthError> {
  let github_client_id = SecretStore::get(secrets, "GITHUB_CLIENT_ID")
    .ok_or_else(|| AuthError::new("Missing expected ENV_VAR: GITHUB_CLIENT_ID"))?;
  let github_client_secret = SecretStore::get(secrets, "GITHUB_CLIENT_SECRET")
    .ok_or_else(|| AuthError::new("Missing expected ENV_VAR: GITHUB_CLIENT_SECRET"))?;

  let http_client = Client::new();
  let response = http_client
    .post(format!("https://api.github.com/applications/{github_client_id}/token"))
    .header("Accept", "application/vnd.github+json")
    .header("content-type", "application/json")
    .header("User-Agent", "rss-reader-service")
    .basic_auth(github_client_id, Some(github_client_secret))
    .body(format!("{{\"access_token\":\"{access_token}\"}}"))
    .send()
    .await
    .map_err(|e| AuthError::new(&format!("Network error: {}", e)))?;

  let status_code = &response.status();

  if status_code != &StatusCode::OK {
    return Err(AuthError::new("Unable to verify user credential!"));
  }

  let response_text: String = response.text().await.map_err(|e| AuthError::new(&format!("Error reading response body: {}", e)))?;

  let token_info: GitHubTokenCheck = serde_json::from_str(&response_text)
    .map_err(|e| AuthError::new(&format!("Error decoding JSON: {}", e)))?;

  let now = Utc::now();
  let token_created = NaiveDateTime::parse_from_str(&token_info.created_at, "%Y-%m-%dT%H:%M:%SZ")
    .map_err(|e| AuthError::new(&format!("Unable to parse token created_at date: {}", e)))?;

  // Check if token was created more than an hour ago, and invalidate
  if token_created + Duration::hours(1) < now.naive_utc() {
    invalidate_expired_token(secrets, access_token).await?;
    return Err(AuthError::new("Expired access token! Token invalidated..."))
  }

  Ok(token_info.user.id.to_string())
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

  let access_token = bearer_token.replace("Bearer ", "");
  let user_id = fetch_github_user_id(&state.secrets, &access_token)
    .await
    .map_err(|e| AuthError::new(&format!("Unable to fetch user credential: {:?}", e)))?;

  let admin_user_id = SecretStore::get(&state.secrets, "GITHUB_USER_ID")
    .ok_or_else(|| AuthError::new("Missing expected ENV_VAR: GITHUB_USER_ID"))?;
  
  if !user_id.eq(&admin_user_id) {
    return Err(AuthError::new("Unauthorized User Action"));
  }

  let req = Request::from_parts(parts, body);
  let response = next.run(req).await;

  Ok(response)
}