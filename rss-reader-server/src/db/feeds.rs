use std::env;

use rusqlite::Connection;

pub struct Feed {
    pub title: String,
    pub url: String,
    pub category: String,
}

pub fn get_feeds() -> Vec<Feed> {
    vec![Feed {
        title: String::from("Node.js Blog"),
        url: String::from("https://nodejs.org/en/feed/blog.xml"),
        category: String::from("Code"),
    }]
}

fn get_current_working_dir() -> String {
    let res = env::current_dir();
    match res {
        Ok(path) => path.display().to_string(),
        Err(_) => "Unable to get current wokring dir - db".to_string(),
    }
}

pub fn setup_feeds_db() -> Connection {
    println!("{}", get_current_working_dir());
    let conn = Connection::open(format!(
        "{}{}",
        get_current_working_dir(),
        "/src/db/feeds.db"
    ))
    .expect("DB - Feeds: Connection failed");

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

    conn
}
