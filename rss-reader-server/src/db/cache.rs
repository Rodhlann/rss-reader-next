use rusqlite::Connection;

pub fn setup_cache() -> Connection {
    let conn = Connection::open_in_memory().expect("Cache: Failed to create connection");

    conn.execute(
        "CREATE TABLE cache (
            data BLOB,
            date_added DATETIME DEFAULT CURRENT_TIMESTAMP
        )",
        (),
    )
    .expect("Cache: Failed to create table");

    conn
}
