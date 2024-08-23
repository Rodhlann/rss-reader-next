use axum::{http::StatusCode, response::IntoResponse};
use serde::{Deserialize, Serialize};
use sqlx::{PgPool, FromRow};

#[derive(Serialize, Deserialize, Debug)]
pub struct FeedInput {
    pub name: String,
    pub url: String,
    pub category: String,
}

#[derive(FromRow, Serialize, Deserialize, Debug)]
pub struct Feed {
  pub id: i32,
  pub name: String,
  pub url: String,
  pub category: String,
}

pub struct FeedDataSource {
  db: PgPool
}

impl FeedDataSource {
  pub fn new(db: PgPool) -> Self {
    Self {
      db
    }
  }

  pub async fn get_feeds(self) -> Result<Vec<Feed>, impl IntoResponse> {
    let res = match sqlx::query_as::<_, Feed>(
      "SELECT feeds.id, feeds.name, feeds.url, categories.name AS category
      FROM feeds
      INNER JOIN categories
      ON
      feeds.category_id = categories.id;")
      .fetch_all(&self.db)
      .await {
        Ok(res) => res,
        Err(e) => {
          return Err((StatusCode::INTERNAL_SERVER_ERROR, e.to_string()));
        }
      };
    
    Ok(res)
  }

  pub async fn create_feed(self, feed: FeedInput) -> Result<impl IntoResponse, impl IntoResponse> {
    if let Err(e) = sqlx::query(
      "WITH category_id_lookup AS (
        INSERT INTO categories (name)
        VALUES ($3)
        ON CONFLICT (name) DO NOTHING
        RETURNING id AS category_id
      )
      INSERT INTO feeds (name, url, category_id)
      SELECT $1, $2, category_id
      FROM category_id_lookup;")
      .bind(feed.name)
      .bind(feed.url)
      .bind(feed.category)
      .execute(&self.db)
      .await {
          return Err(
              (StatusCode::INTERNAL_SERVER_ERROR,
              format!("Error while inserting a feed: {e}"))
          );
      }
    
    Ok(())
  }

  pub async fn delete_feed(self, id: i32) -> Result<impl IntoResponse, impl IntoResponse> {
    if let Err(e) = sqlx::query_as::<_,Feed>("DELETE FROM feeds WHERE id = $1")
      .bind(id)
      .fetch_all(&self.db)
      .await {
        return Err((
          StatusCode::INTERNAL_SERVER_ERROR,
          format!("Error while deleting feed: {e}"))
        );
      }
    Ok(StatusCode::OK)
  }
}