use auth::auth_middleware;
use axum::{middleware, routing::{delete, get}, Router};
use service::{delete_feed, get_raw_feeds, get_rss_feeds, schedule_cache_clear};
use shuttle_runtime::SecretStore;
use sqlx::PgPool;

mod auth;
mod db;
mod service;

use crate::service::create_feed;

#[derive(Clone)]
pub struct AppState {
    db: PgPool,
    secrets: SecretStore
}

#[shuttle_runtime::main]
async fn main(
    #[shuttle_shared_db::Postgres] db: PgPool,
    #[shuttle_runtime::Secrets] secrets: shuttle_runtime::SecretStore
) -> shuttle_axum::ShuttleAxum {
    sqlx::migrate!().run(&db).await.expect("Migrations failed :(");

    // Confirm ENV_VARS are set
    let _ = SecretStore::get(&secrets, "GITHUB_USER_ID")
        .ok_or_else(|| panic!("Missing expected ENV_VAR: GITHUB_USER_ID"));

    let state = AppState { db, secrets };

    schedule_cache_clear(&state.db).await
        .unwrap_or_else(|e| panic!("Failed to start cache clear job: {}", e));

    let unprotected_routes = Router::new()
        .route("/feeds", 
            get(get_rss_feeds)
        );
    
    let protected_routes = Router::new()
        .route("/admin", 
            get(get_raw_feeds)
            .post(create_feed)
        )
        .route("/admin/:id",
            delete(delete_feed)
        )
        .layer(middleware::from_fn_with_state(state.clone(), auth_middleware));

    let routes = Router::new()
        .merge(unprotected_routes)
        .merge(protected_routes)
        .with_state(state);

    Ok(routes.into())
}
