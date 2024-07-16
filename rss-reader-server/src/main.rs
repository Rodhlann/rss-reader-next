mod db;
mod feeds;
mod server;

use crate::db::setup_cache;
use crate::db::setup_feeds_db;
use crate::server::app;

#[tokio::main]
async fn main() {
    let feeds_conn = setup_feeds_db();
    let cache_conn = setup_cache();

    println!("Server listening at http://localhost:3000");

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app()).await.unwrap();
}
