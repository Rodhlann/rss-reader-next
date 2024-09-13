use std::fmt;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::{PgPool, FromRow};

#[derive(Debug)]
pub enum CacheError {
    Database(sqlx::Error),
}

impl fmt::Display for CacheError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CacheError::Database(err) => write!(f, "Database error: {}", err),
        }
    }
}

impl std::error::Error for CacheError {}

impl From<sqlx::Error> for CacheError {
    fn from(error: sqlx::Error) -> Self {
        CacheError::Database(error)
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CacheInput {
    pub name: String,
    pub xml_string: String,
}

#[derive(FromRow, Serialize, Deserialize, Debug)]
pub struct CacheValue {
  pub name: String,
  pub xml_string: String,
  pub created_date: DateTime<Utc>,
}

pub struct CacheDataSource {
  db: PgPool
}

impl CacheDataSource {
  pub fn new(db: &PgPool) -> Self {
    Self {
      db: db.clone()
    }
  }

  pub async fn get_cached_value(self, name: String) -> Result<Option<CacheValue>, CacheError> {    
    println!("Fetching cached feed: {}", name);

    let res = match sqlx::query_as::<_, CacheValue>(
      "SELECT * FROM cache where name = $1;")
      .bind(&name)
      .fetch_optional(&self.db)
      .await {
        Ok(res) => res,
        Err(e) => {
          return Err(CacheError::Database(e));
        }
      };
    
    Ok(res)
  }

  pub async fn cache_value(self, cache_value: CacheInput) -> Result<(), CacheError> {
    println!("Caching feed: {}", cache_value.name);

    if let Err(e) = sqlx::query(
        "INSERT INTO cache (name, xml_string) VALUES ($1, $2);"
    )
    .bind(&cache_value.name)
    .bind(&cache_value.xml_string)
    .execute(&self.db)
    .await
    {
        return Err(CacheError::Database(e));
    }

    Ok(())
  }

  pub async fn clear_cache(self) -> Result<(), CacheError> {
    let stale_cache = match sqlx::query_as::<_, CacheValue>(
      "SELECT * FROM cache 
        WHERE created_date < NOW() - INTERVAL '10 minutes';")
      .fetch_all(&self.db)
      .await {
        Ok(res) => res,
        Err(e) => {
          return Err(CacheError::Database(e));
        }
      };

    let stale_names: Vec<String> = stale_cache.iter().map(|c| c.name.clone()).collect();
    if !stale_names.is_empty() {
      println!("Clearing stale cache items: [{}]", stale_names.join(", "));

      if let Err(e) = sqlx::query_as::<_, CacheValue>(
        "DELETE FROM cache
          WHERE created_date < NOW() - INTERVAL '10 minutes';"
      )
        .fetch_all(&self.db)
        .await {
          return Err(CacheError::Database(e));
        }
    }

    Ok(())
  }
}