use std::path::PathBuf;

use rusqlite::Connection;
use sde2sqlite;

pub mod universe;

pub(crate) const EVE_DB_FILE: &str = "EVE.db";

pub fn update_database() {
    sde2sqlite::update();
}

pub(crate) fn connect_db() -> Connection {
    let db: PathBuf = PathBuf::from(EVE_DB_FILE);
    if !db.exists() {
        panic!("Database file is missing, use eve_orm::update_database() to generate it")
    }
    let connection: Connection;
    match Connection::open(db) {
        Ok(result) => connection = result,
        Err(error) => panic!("ERROR when opening database: {:#?}", error),
    }
    connection
}
