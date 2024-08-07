use std::env;
use serde::Deserialize;
use rusqlite::{params, Connection, Error, Result};

#[derive(Deserialize, Debug)]
pub struct Feed {
    pub title: String,
    pub url: String,
    pub category: String,
}

#[derive(Debug)]
struct DB_Feed {
    pub title: String,
    pub url: String,
    pub category_id: i32,
}

pub fn get_feeds() -> Result<Vec<Feed>> {
    // vec![Feed {
    //     title: String::from("Node.js Blog"),
    //     url: String::from("https://nodejs.org/en/feed/blog.xml"),
    //     category: String::from("Code"),
    // }]
    let conn = open_feeds_conn();

    let mut stmt = conn.prepare(
        "SELECT title, url, category_id FROM feed"
    )?;

    let feeds = stmt.query_map([], |row| {
        Ok(Feed {
            title: row.get(0)?,
            url: row.get(1)?,
            category: "0".to_string()
        })
    })?;

    let mut out: Vec<Feed> = Vec::new();

    for feed in feeds {
        out.push(feed?);
    }

    // let mut stmt = conn.prepare(
    //     "SELECT id FROM category 
    //         WHERE name = ?1")?;
    
    // let mut rows = stmt.query(params![feed.category])?;

    Ok(out)
}

pub fn persist_feed(feed: Feed) -> Result<()> {
    let conn = open_feeds_conn();

    conn.execute(
        "INSERT INTO category (name)
            VALUES (?1)",
            params![feed.category]
    )?;

    let mut stmt = conn.prepare(
        "SELECT id FROM category 
            WHERE name = ?1")?;
    
    let mut rows = stmt.query(params![feed.category])?;

    let category_id: i32 = match rows.next()? {
        Some(row) => row.get(0)?,
        None => return Err(Error::QueryReturnedNoRows),
    };
    
    conn.execute(
        "INSERT INTO feed (title, url, category_id) 
            VALUES (?1, ?2, ?3)", 
            (feed.title, feed.url, category_id)
        )?;

    Ok(())
}

fn get_current_working_dir() -> String {
    let res = env::current_dir();
    match res {
        Ok(path) => path.display().to_string(),
        Err(_) => "Unable to get current working dir - db".to_string(),
    }
}

fn open_feeds_conn() -> Connection {
    Connection::open(format!(
        "{}{}",
        get_current_working_dir(),
        "/src/db/feeds.db"
    ))
    .expect("DB - Feeds: Connection failed")
}

pub fn setup_feeds_db() -> () {
    let conn = open_feeds_conn();

    conn.execute(
        "CREATE TABLE IF NOT EXISTS category (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            name TEXT NOT NULL UNIQUE,
            date_added DATETIME DEFAULT CURRENT_TIMESTAMP
        )",
        (),
    )
    .expect("DB - Feeds: Table setup failed - Category");

    conn.execute(
        "CREATE TABLE IF NOT EXISTS feed (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            title TEXT NOT NULL UNIQUE,
            url TEXT NOT NULL UNIQUE,
            category_id INTEGER,
            CONSTRAINT fk_category
                FOREIGN KEY (category_id)
                REFERENCES category(id)
        )",
        (),
    )
    .expect("DB - Feeds: Table setup failed - Feed");
}
