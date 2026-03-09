use std::collections::HashMap;

use crate::connect_db;
use rusqlite::{Connection, Statement};

#[derive(Debug, Clone, PartialEq)]
pub struct Galaxy {
    pub id: i64,
    pub name: String,
}

pub fn get_all() -> HashMap<i64, Galaxy> {
    let mut galaxies: HashMap<i64, Galaxy> = HashMap::new();
    let connection: Connection = connect_db();
    let mut stmt: Statement<'_>;
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
        let id: i64;
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

pub fn get(id: &i64) -> Option<Galaxy> {
    let galaxy: Galaxy;
    let connection: Connection = connect_db();
    let mut stmt: Statement<'_>;
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

pub fn search(search_query: &str) -> Vec<Galaxy> {
    let mut galaxies: Vec<Galaxy> = Vec::new();
    let galaxies_map: HashMap<i64, Galaxy> = get_all();
    let search_query: &str = &search_query.to_lowercase().replace("-", "");
    if search_query.chars().count() < 3 {
        return galaxies;
    }
    for galaxy_map in galaxies_map {
        let galaxy_name: &str = &galaxy_map.1.name.to_lowercase().replace("-", "");
        if galaxy_name.starts_with(search_query) {
            galaxies.push(galaxy_map.1.clone());
        }
    }
    galaxies.sort_by(|s1, s2| s1.name.cmp(&s2.name));

    galaxies
}

#[cfg(test)]
mod tests {
    use crate::universe::galaxy::{self, Galaxy};

    #[test]
    fn get_ok() {
        let galaxy: Option<Galaxy> = galaxy::get(&1);
        assert_eq!(
            galaxy,
            Some(Galaxy {
                id: 1,
                name: "NEW EDEN".to_string()
            })
        );
    }

    #[test]
    fn get_none() {
        let galaxy: Option<Galaxy> = galaxy::get(&1321412421);
        assert_eq!(galaxy, None);
    }

    #[test]
    fn search_exact() {
        let galaxy: Vec<Galaxy> = galaxy::search("NEW EDEN");
        assert_eq!(
            galaxy[0],
            Galaxy {
                id: 1,
                name: "NEW EDEN".to_string()
            }
        );
    }

    #[test]
    fn search() {
        let galaxy: Vec<Galaxy> = galaxy::search("nEw E");
        assert_eq!(
            galaxy[0],
            Galaxy {
                id: 1,
                name: "NEW EDEN".to_string()
            }
        );
    }

    #[test]
    fn search_not_found() {
        let galaxy: Vec<Galaxy> = galaxy::search("dsafasfa");
        assert_eq!(galaxy.len(), 0);
    }

    #[test]
    fn search_under_3_chars() {
        let galaxy: Vec<Galaxy> = galaxy::search("NE-");
        assert_eq!(galaxy.len(), 0);
    }
}
