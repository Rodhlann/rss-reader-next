use std::{fmt, sync::Arc, time::Duration};

use sqlx::PgPool;
use tokio_cron_scheduler::{Job, JobScheduler, JobSchedulerError};
use tokio::task;

use crate::db::{CacheDataSource, CacheValue};

#[derive(Debug)]
pub enum CacheError {
    Service(String),
}

impl fmt::Display for CacheError {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
      match self {
          CacheError::Service(msg) => write!(f, "Cache service error: {}", msg),
      }
  }
}

impl std::error::Error for CacheError {}

pub async fn fetch_cached(cache_name: &str, db: &PgPool) -> Result<Option<CacheValue>, CacheError> {
  let cache = CacheDataSource::new(&db.to_owned());

  let cached = cache.get_cached_value(cache_name.to_string()).await
    .map_err(|e| CacheError::Service(e.to_string()))?;

  Ok(cached)
}

pub async fn schedule_cache_clear(db: &PgPool) -> Result<(), JobSchedulerError> {
  let db = Arc::new(db.clone());
  let sched = JobScheduler::new().await?;

  println!("Scheduling cache clear job");

  // Schedule cache clear every 5 minutes, deletes records greater than 10 minutes old
  sched.add(
    Job::new_repeated(Duration::from_secs(300), move |_uuid, _l| {
      println!("Running cache clear job");
      let db = Arc::clone(&db);
      task::spawn(async move {
        let cache = CacheDataSource::new(&db);
        if let Err(e) = cache.clear_cache().await {
          eprintln!("Failed to clear cache: {}", e);
        }
      });
    })?
  ).await?;

  sched.start().await?;

  Ok(())
}