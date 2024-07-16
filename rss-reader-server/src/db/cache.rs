use rusqlite::Connection;

pub fn setup_cache() -> Connection {
    let conn = Connection::open_in_memory().expect("Cache: Failed to create connection");
    conn
}
