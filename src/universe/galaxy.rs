use std::collections::HashMap;

use crate::connect_db;
use rusqlite::Connection;

#[derive(Debug, Clone)]
pub struct Galaxy {
    pub id: u64,
    pub name: String,
}

pub fn get_all() -> HashMap<u64, Galaxy> {
    let mut galaxies: HashMap<u64, Galaxy> = HashMap::new();
    let connection: Connection = connect_db();
    let mut stmt;
    match connection.prepare(
        "SELECT
             galaxy.id,
             galaxy.name
            FROM galaxy;",
    ) {
        Ok(result) => stmt = result,
        Err(error) => panic!("ERROR when preparing query statement: {:#?}", error),
    }
    let galaxies_iter;
    match stmt.query_map(rusqlite::params![], |row| {
        let id: u64;
        match row.get(0) {
            Ok(value) => id = value,
            Err(error) => panic!("ERROR when retrieving value: {:#?}", error),
        }
        let name: String;
        match row.get(1) {
            Ok(value) => name = value,
            Err(error) => panic!("ERROR when retrieving value: {:#?}", error),
        }
        Ok(Galaxy { id, name })
    }) {
        Ok(value) => galaxies_iter = value,
        Err(error) => panic!("ERROR when retrieving value: {:#?}", error),
    }
    for galaxy in galaxies_iter {
        match galaxy {
            Ok(galaxy) => galaxies.insert(galaxy.id, galaxy),
            Err(error) => panic!("ERROR when retrieving value: {:#?}", error),
        };
    }

    galaxies
}

pub fn get(id: &u64) -> Option<Galaxy> {
    let galaxy: Galaxy;
    let connection: Connection = connect_db();
    let mut stmt;
    match connection.prepare(
        "SELECT
             galaxy.name
            FROM galaxy
            WHERE galaxy.id = ?1;",
    ) {
        Ok(result) => stmt = result,
        Err(error) => panic!("ERROR when preparing query statement: {:#?}", error),
    }
    let mut rows;
    match stmt.query(rusqlite::params![id]) {
        Ok(result) => rows = result,
        Err(error) => panic!("ERROR when executing query: {:#?}", error),
    }
    match rows.next() {
        Ok(rows) => match rows {
            Some(row) => {
                let name: String;
                match row.get(0) {
                    Ok(value) => name = value,
                    Err(error) => panic!("ERROR when retrieving value: {:#?}", error),
                }
                galaxy = Galaxy { id: *id, name }
            }
            None => return None,
        },
        Err(error) => panic!("ERROR when getting row: {:#?}", error),
    }

    Some(galaxy)
}

pub fn search(_name: &str) -> Option<Galaxy> {
    todo!()
}
