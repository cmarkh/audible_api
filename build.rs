use include_dir::{include_dir, Dir};
use rusqlite::Connection;
use rusqlite_migration::Migrations;

const AUTH_DB_PATH: &str = "auth.db";
static MIGRATIONS_DIR: &Dir = &include_dir!("migrations");

fn main() {
    let mut conn = Connection::open(AUTH_DB_PATH).unwrap();

    Migrations::from_directory(MIGRATIONS_DIR)
        .unwrap()
        .to_latest(&mut conn)
        .unwrap();
}

