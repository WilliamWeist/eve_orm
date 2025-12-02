use std::collections::HashMap;

use rusqlite::Connection;

use crate::{
    connect_db,
    universe::{
        Position,
        constellation::{self, Constellation},
        stargate::{self, Stargate},
    },
};

#[derive(Debug, Clone)]
pub struct System {
    pub id: u64,
    pub name: String,
    pub security_status: f64,
    pub position: Position,
    pub constellation: Constellation,
    pub stargates: Vec<Stargate>,
}

pub fn get_all() -> HashMap<u64, System> {
    let constellations: HashMap<u64, Constellation> = constellation::get_all();
    let stargates_map: HashMap<u64, Vec<Stargate>> = stargate::get_all();
    let mut systems: HashMap<u64, System> = HashMap::new();
    let connection: Connection = connect_db();
    let mut stmt;
    match connection.prepare(
        "SELECT
             system.id,
             name.en AS name,
             system.security_status,
             position.x AS x,
             position.y AS y,
             position.z AS z,
             system.constellation_id
            FROM system
            INNER JOIN name ON system.id = name.entity_id
            INNER JOIN position ON system.id = position.entity_id;",
    ) {
        Ok(result) => stmt = result,
        Err(error) => panic!("ERROR when preparing query statement: {:#?}", error),
    }
    let systems_iter;
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
        let security_status: f64;
        match row.get(2) {
            Ok(value) => security_status = value,
            Err(error) => panic!("ERROR when retrieving value: {:#?}", error),
        }
        let x: f64;
        match row.get(3) {
            Ok(value) => x = value,
            Err(error) => panic!("ERROR when retrieving value: {:#?}", error),
        }
        let y: f64;
        match row.get(4) {
            Ok(value) => y = value,
            Err(error) => panic!("ERROR when retrieving value: {:#?}", error),
        }
        let z: f64;
        match row.get(5) {
            Ok(value) => z = value,
            Err(error) => panic!("ERROR when retrieving value: {:#?}", error),
        }
        let constellation_id: u64;
        match row.get(6) {
            Ok(value) => constellation_id = value,
            Err(error) => panic!("ERROR when retrieving value: {:#?}", error),
        }
        let constellation: Constellation;
        match constellations.get(&constellation_id) {
            Some(value) => constellation = value.clone(),
            None => todo!(),
        }
        let stargates: Vec<Stargate>;
        match stargates_map.get(&id) {
            Some(value) => stargates = value.clone(),
            None => stargates = Vec::new(),
        }
        Ok(System {
            id,
            name,
            security_status,
            position: Position { x, y, z },
            constellation,
            stargates,
        })
    }) {
        Ok(value) => systems_iter = value,
        Err(error) => panic!("ERROR when retrieving value: {:#?}", error),
    }
    for system in systems_iter {
        match system {
            Ok(system) => systems.insert(system.id, system),
            Err(error) => panic!("ERROR when retrieving value: {:#?}", error),
        };
    }

    systems
}

pub fn get(id: &u64) -> Option<System> {
    let system: System;
    let connection: Connection = connect_db();
    let mut stmt;
    match connection.prepare(
        "SELECT
             name.en AS name,
             system.security_status,
             position.x AS x,
             position.y AS y,
             position.z AS z,
             system.constellation_id
            FROM system
            INNER JOIN name ON system.id = name.entity_id
            INNER JOIN position ON system.id = position.entity_id
            WHERE system.id = ?1;",
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
                let security_status: f64;
                match row.get(1) {
                    Ok(value) => security_status = value,
                    Err(error) => panic!("ERROR when retrieving value: {:#?}", error),
                }
                let x: f64;
                match row.get(2) {
                    Ok(value) => x = value,
                    Err(error) => panic!("ERROR when retrieving value: {:#?}", error),
                }
                let y: f64;
                match row.get(3) {
                    Ok(value) => y = value,
                    Err(error) => panic!("ERROR when retrieving value: {:#?}", error),
                }
                let z: f64;
                match row.get(4) {
                    Ok(value) => z = value,
                    Err(error) => panic!("ERROR when retrieving value: {:#?}", error),
                }
                let constellation_id: u64;
                match row.get(5) {
                    Ok(value) => constellation_id = value,
                    Err(error) => panic!("ERROR when retrieving value: {:#?}", error),
                }
                let constellation: Constellation;
                match constellation::get(&constellation_id) {
                    Some(value) => constellation = value,
                    None => panic!(
                        "ERROR: System id: {} is not assigned to any constellation",
                        id
                    ),
                }
                let stargates: Vec<Stargate> = stargate::get(id);
                system = System {
                    id: *id,
                    name,
                    security_status,
                    position: Position { x, y, z },
                    constellation,
                    stargates,
                }
            }
            None => return None,
        },
        Err(error) => panic!("ERROR when getting row: {:#?}", error),
    }
    Some(system)
}

pub fn search(search_query: &str, cache: Option<&HashMap<u64, System>>) -> Vec<System> {
    let mut systems: Vec<System> = Vec::new();
    let systems_map: HashMap<u64, System>;
    match cache {
        Some(cache) => systems_map = cache.clone(),
        None => systems_map = get_all(),
    }
    let search_query: &str = &search_query.to_lowercase().replace("-", "");
    for system_map in systems_map {
        let system_name: &str = &system_map.1.name.to_lowercase().replace("-", "");
        if system_name.starts_with(search_query) {
            systems.push(system_map.1.clone());
        }
    }
    systems.sort_by(|s1, s2| s1.name.cmp(&s2.name));

    systems
}
