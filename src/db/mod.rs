pub mod auth;

use r2d2_sqlite::SqliteConnectionManager;

pub type Pool = r2d2::Pool<SqliteConnectionManager>;

pub fn new_pool() -> Result<Pool, r2d2::Error> {
    let manager = SqliteConnectionManager::file("file.db");
    let pool = r2d2::Pool::new(manager)?;
    Ok(pool)
}
