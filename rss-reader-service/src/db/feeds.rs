use axum::{http::StatusCode, response::IntoResponse, Json};
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
    println!("Creating new feed: {:?}", feed);

    let new_category_id = sqlx::query_scalar(
        "INSERT INTO categories (name)
        VALUES ($1)
        ON CONFLICT (name) DO NOTHING
        RETURNING id"
    )
    .bind(&feed.category)
    .fetch_optional(&self.db)
    .await;

    let category_id: i32 = match new_category_id {
        Ok(Some(id)) => id,
        Ok(None) => {
            sqlx::query_scalar("SELECT id FROM categories WHERE name = $1")
                .bind(&feed.category)
                .fetch_one(&self.db)
                .await
                .map_err(|e| {
                    (
                        StatusCode::INTERNAL_SERVER_ERROR,
                        format!("Error fetching existing category ID: {e}"),
                    )
                })?
        },
        Err(e) => {
            return Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Error while inserting category: {e}"),
            ));
        }
    };

    if let Err(e) = sqlx::query(
        "INSERT INTO feeds (name, url, category_id)
        VALUES ($1, $2, $3)"
    )
    .bind(&feed.name)
    .bind(&feed.url)
    .bind(category_id)
    .execute(&self.db)
    .await
    {
        return Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Error while inserting a feed: {e}"),
        ));
    }

    let res = match sqlx::query_as::<_, Feed>(
      "SELECT feeds.id, feeds.name, feeds.url, categories.name AS category
      FROM feeds
      INNER JOIN categories
      ON
      feeds.category_id = categories.id
      WHERE feeds.name = $1;")
      .bind(feed.name)
      .fetch_one(&self.db)
      .await {
        Ok(res) => res,
        Err(e) => {
          return Err((StatusCode::INTERNAL_SERVER_ERROR, e.to_string()));
        }
      };
    
    Ok(Json(res))
  }

  pub async fn delete_feed(self, id: i32) -> Result<impl IntoResponse, impl IntoResponse> {
    println!("Deleting feed: {}", id);

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