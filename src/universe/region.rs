use std::collections::HashMap;

use rusqlite::Connection;

use crate::connect_db;
use crate::universe::galaxy::{self, Galaxy};

#[derive(Debug, Clone, PartialEq)]
pub struct Region {
    pub id: u64,
    pub name: String,
    pub galaxy: Galaxy,
}

pub fn get_all() -> HashMap<u64, Region> {
    let galaxies: HashMap<u64, Galaxy> = galaxy::get_all();
    let mut regions: HashMap<u64, Region> = HashMap::new();
    let connection: Connection = connect_db();
    let mut stmt;
    match connection.prepare(
        "SELECT
             region.id,
             name.en AS name,
             region.galaxy_id
            FROM region
            INNER JOIN name ON region.id = name.entity_id;",
    ) {
        Ok(result) => stmt = result,
        Err(error) => panic!("ERROR when preparing query statement: {:#?}", error),
    }
    let regions_iter;
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
        let galaxy_id: u64;
        match row.get(2) {
            Ok(value) => galaxy_id = value,
            Err(error) => panic!("ERROR when retrieving value: {:#?}", error),
        }
        let galaxy: Galaxy;
        match galaxies.get(&galaxy_id) {
            Some(value) => galaxy = value.clone(),
            None => todo!(),
        }
        Ok(Region { id, name, galaxy })
    }) {
        Ok(value) => regions_iter = value,
        Err(error) => panic!("ERROR when retrieving value: {:#?}", error),
    }
    for region in regions_iter {
        match region {
            Ok(region) => regions.insert(region.id, region),
            Err(error) => panic!("ERROR when retrieving value: {:#?}", error),
        };
    }

    regions
}

pub fn get(id: &u64) -> Option<Region> {
    let region: Region;
    let connection: Connection = connect_db();
    let mut stmt;
    match connection.prepare(
        "SELECT
             name.en AS name,
             region.galaxy_id
            FROM region
            INNER JOIN name ON region.id = name.entity_id
            WHERE region.id = ?1;",
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
                let galaxy_id: u64;
                match row.get(1) {
                    Ok(value) => galaxy_id = value,
                    Err(error) => panic!("ERROR when retrieving value: {:#?}", error),
                }
                let galaxy: Galaxy;
                match galaxy::get(&galaxy_id) {
                    Some(value) => galaxy = value,
                    None => panic!("ERROR: Region id: {} is not assigned to any galaxy", id),
                }
                region = Region {
                    id: *id,
                    name,
                    galaxy,
                }
            }
            None => return None,
        },
        Err(error) => panic!("ERROR when getting row: {:#?}", error),
    }
    Some(region)
}

pub fn search(search_query: &str) -> Vec<Region> {
    let mut regions: Vec<Region> = Vec::new();
    let regions_map: HashMap<u64, Region> = get_all();
    let search_query: &str = &search_query.to_lowercase().replace("-", "");
    if search_query.chars().count() < 3 {
        return regions;
    }
    for region_map in regions_map {
        let region_name: &str = &region_map.1.name.to_lowercase().replace("-", "");
        if region_name.starts_with(search_query) {
            regions.push(region_map.1.clone());
        }
    }
    regions.sort_by(|s1, s2| s1.name.cmp(&s2.name));

    regions
}

#[cfg(test)]
mod tests {
    use crate::universe::{
        galaxy::Galaxy,
        region::{self, Region},
    };

    #[test]
    fn get_ok() {
        let region: Option<Region> = region::get(&10000001);
        assert_eq!(
            region,
            Some(Region {
                id: 10000001,
                name: "Derelik".to_string(),
                galaxy: Galaxy {
                    id: 1,
                    name: "NEW EDEN".to_string()
                }
            })
        );
    }

    #[test]
    fn get_none() {
        let region: Option<Region> = region::get(&1321412421);
        assert_eq!(region, None);
    }

    #[test]
    fn search_exact() {
        let region: Vec<Region> = region::search("Derelik");
        assert_eq!(
            region[0],
            Region {
                id: 10000001,
                name: "Derelik".to_string(),
                galaxy: Galaxy {
                    id: 1,
                    name: "NEW EDEN".to_string()
                }
            }
        );
    }

    #[test]
    fn search() {
        let region: Vec<Region> = region::search("dEr");
        assert_eq!(
            region[0],
            Region {
                id: 10000001,
                name: "Derelik".to_string(),
                galaxy: Galaxy {
                    id: 1,
                    name: "NEW EDEN".to_string()
                }
            }
        );
    }

    #[test]
    fn search_not_found() {
        let region: Vec<Region> = region::search("dsafasfa");
        assert_eq!(region.len(), 0);
    }

    #[test]
    fn search_under_3_chars() {
        let region: Vec<Region> = region::search("DE-");
        assert_eq!(region.len(), 0);
    }
}
