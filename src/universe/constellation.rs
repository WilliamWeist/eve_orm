use std::collections::HashMap;

use rusqlite::Connection;

use crate::connect_db;
use crate::universe::region::{self, Region};

#[derive(Debug, Clone, PartialEq)]
pub struct Constellation {
    pub id: u64,
    pub name: String,
    pub region: Region,
}

pub fn get_all() -> HashMap<u64, Constellation> {
    let regions: HashMap<u64, Region> = region::get_all();
    let mut constellations: HashMap<u64, Constellation> = HashMap::new();
    let connection: Connection = connect_db();
    let mut stmt;
    match connection.prepare(
        "SELECT
             constellation.id,
             name.en AS name,
             constellation.region_id
            FROM constellation
            INNER JOIN name ON constellation.id = name.entity_id;",
    ) {
        Ok(result) => stmt = result,
        Err(error) => panic!("ERROR when preparing query statement: {:#?}", error),
    }
    let constellations_iter;
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
        let region_id: u64;
        match row.get(2) {
            Ok(value) => region_id = value,
            Err(error) => panic!("ERROR when retrieving value: {:#?}", error),
        }
        let region: Region;
        match regions.get(&region_id) {
            Some(value) => region = value.clone(),
            None => todo!(),
        }
        Ok(Constellation { id, name, region })
    }) {
        Ok(value) => constellations_iter = value,
        Err(error) => panic!("ERROR when retrieving value: {:#?}", error),
    }
    for constellation in constellations_iter {
        match constellation {
            Ok(constellation) => constellations.insert(constellation.id, constellation),
            Err(error) => panic!("ERROR when retrieving value: {:#?}", error),
        };
    }

    constellations
}

pub fn get(id: &u64) -> Option<Constellation> {
    let constellation: Constellation;
    let connection: Connection = connect_db();
    let mut stmt;
    match connection.prepare(
        "SELECT
             name.en AS name,
             constellation.region_id
            FROM constellation
            INNER JOIN name ON constellation.id = name.entity_id
            WHERE constellation.id = ?1;",
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
                let region_id: u64;
                match row.get(1) {
                    Ok(value) => region_id = value,
                    Err(error) => panic!("ERROR when retrieving value: {:#?}", error),
                }
                let region: Region;
                match region::get(&region_id) {
                    Some(value) => region = value,
                    None => panic!(
                        "ERROR: Constellation id: {} is not assigned to any region",
                        id
                    ),
                }
                constellation = Constellation {
                    id: *id,
                    name,
                    region,
                }
            }
            None => return None,
        },
        Err(error) => panic!("ERROR when getting row: {:#?}", error),
    }
    Some(constellation)
}

pub fn search(search_query: &str) -> Vec<Constellation> {
    let mut constellations: Vec<Constellation> = Vec::new();
    let constellations_map: HashMap<u64, Constellation> = get_all();
    let search_query: &str = &search_query.to_lowercase().replace("-", "");
    if search_query.chars().count() < 3 {
        return constellations;
    }
    for constellation_map in constellations_map {
        let constellation_name: &str = &constellation_map.1.name.to_lowercase().replace("-", "");
        if constellation_name.starts_with(search_query) {
            constellations.push(constellation_map.1.clone());
        }
    }
    constellations.sort_by(|s1, s2| s1.name.cmp(&s2.name));

    constellations
}

#[cfg(test)]
mod tests {
    use crate::universe::{
        constellation::{self, Constellation},
        galaxy::Galaxy,
        region::Region,
    };

    #[test]
    fn get_ok() {
        let constellation: Option<Constellation> = constellation::get(&20000001);
        assert_eq!(
            constellation,
            Some(Constellation {
                id: 20000001,
                name: "San Matar".to_string(),
                region: Region {
                    id: 10000001,
                    name: "Derelik".to_string(),
                    galaxy: Galaxy {
                        id: 1,
                        name: "NEW EDEN".to_string()
                    }
                }
            })
        );
    }

    #[test]
    fn get_none() {
        let constellation: Option<Constellation> = constellation::get(&1321412421);
        assert_eq!(constellation, None);
    }

    #[test]
    fn search_exact() {
        let constellation: Vec<Constellation> = constellation::search("San Matar");
        assert_eq!(
            constellation[0],
            Constellation {
                id: 20000001,
                name: "San Matar".to_string(),
                region: Region {
                    id: 10000001,
                    name: "Derelik".to_string(),
                    galaxy: Galaxy {
                        id: 1,
                        name: "NEW EDEN".to_string()
                    }
                }
            }
        );
    }

    #[test]
    fn search() {
        let constellation: Vec<Constellation> = constellation::search("sAn m");
        assert_eq!(
            constellation[0],
            Constellation {
                id: 20000001,
                name: "San Matar".to_string(),
                region: Region {
                    id: 10000001,
                    name: "Derelik".to_string(),
                    galaxy: Galaxy {
                        id: 1,
                        name: "NEW EDEN".to_string()
                    }
                }
            }
        );
    }

    #[test]
    fn search_not_found() {
        let constellation: Vec<Constellation> = constellation::search("dsafasfa");
        assert_eq!(constellation.len(), 0);
    }

    #[test]
    fn search_under_3_chars() {
        let region: Vec<Constellation> = constellation::search("sa-");
        assert_eq!(region.len(), 0);
    }
}
