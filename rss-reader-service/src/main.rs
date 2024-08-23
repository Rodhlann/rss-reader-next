use axum::{routing::{delete, get}, Router};
use service::{delete_feed, get_raw_feeds, get_rss_feeds};
use sqlx::PgPool;

mod service;
mod db;

use crate::service::create_feed;

#[derive(Clone)]
pub struct AppState {
    db: PgPool,
}

#[shuttle_runtime::main]
async fn main(
    #[shuttle_shared_db::Postgres] db: PgPool,
) -> shuttle_axum::ShuttleAxum {
    sqlx::migrate!().run(&db).await.expect("Migrations failed :(");

    let state = AppState { db };

    let router = Router::new()
        .route("/feeds", 
            get(get_rss_feeds)
            .post(create_feed)
        )
        .route("/admin", 
            get(get_raw_feeds)
        )
        .route("/admin/:id",
            delete(delete_feed)
        )
        .with_state(state);

    Ok(router.into())
}
