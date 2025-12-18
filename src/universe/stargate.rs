use std::collections::HashMap;

use rusqlite::Connection;

use crate::connect_db;

#[derive(Debug, Clone, PartialEq)]
pub struct Stargate {
    pub destination_id: u64,
    pub destination_name: String,
}

pub fn get_all() -> HashMap<u64, Vec<Stargate>> {
    let mut stargates_map: HashMap<u64, Vec<Stargate>> = HashMap::new();
    let connection: Connection = connect_db();
    let mut stmt;
    match connection.prepare(
        "SELECT
             v_stargate.from_system_id,
             v_stargate.to_system_id,
             v_stargate.to_system
            FROM v_stargate;",
    ) {
        Ok(result) => stmt = result,
        Err(error) => panic!("ERROR when preparing query statement: {:#?}", error),
    }
    let stargates_iter;
    match stmt.query_map(rusqlite::params![], |row| {
        let from_system_id: u64;
        match row.get(0) {
            Ok(value) => from_system_id = value,
            Err(error) => panic!("ERROR when retrieving value: {:#?}", error),
        }
        let destination_id: u64;
        match row.get(1) {
            Ok(value) => destination_id = value,
            Err(error) => panic!("ERROR when retrieving value: {:#?}", error),
        }
        let destination_name: String;
        match row.get(2) {
            Ok(value) => destination_name = value,
            Err(error) => panic!("ERROR when retrieving value: {:#?}", error),
        }
        Ok((
            from_system_id,
            Stargate {
                destination_id,
                destination_name,
            },
        ))
    }) {
        Ok(value) => stargates_iter = value,
        Err(error) => panic!("ERROR when retrieving value: {:#?}", error),
    }
    for stargate in stargates_iter {
        match stargate {
            Ok(stargate) => {
                let mut stargates: Vec<Stargate>;
                match stargates_map.get(&stargate.0) {
                    Some(values) => stargates = values.to_vec(),
                    None => stargates = Vec::new(),
                }
                stargates.push(stargate.1);
                stargates_map.insert(stargate.0, stargates);
            }
            Err(error) => panic!("ERROR when retrieving value: {:#?}", error),
        };
    }

    stargates_map
}

pub fn get(id: &u64) -> Vec<Stargate> {
    let mut stargates: Vec<Stargate> = Vec::new();
    let connection: Connection = connect_db();
    let mut stmt;
    match connection.prepare(
        "SELECT
             v_stargate.to_system_id,
             v_stargate.to_system
            FROM v_stargate
            WHERE v_stargate.from_system_id = ?1;",
    ) {
        Ok(result) => stmt = result,
        Err(error) => panic!("ERROR when preparing query statement: {:#?}", error),
    }
    let stargates_iter;
    match stmt.query_map(rusqlite::params![id], |row| {
        let destination_id: u64;
        match row.get(0) {
            Ok(value) => destination_id = value,
            Err(error) => panic!("ERROR when retrieving value: {:#?}", error),
        }
        let destination_name: String;
        match row.get(1) {
            Ok(value) => destination_name = value,
            Err(error) => panic!("ERROR when retrieving value: {:#?}", error),
        }
        Ok(Stargate {
            destination_id,
            destination_name,
        })
    }) {
        Ok(value) => stargates_iter = value,
        Err(error) => panic!("ERROR when retrieving value: {:#?}", error),
    }
    for stargate in stargates_iter {
        match stargate {
            Ok(stargate) => stargates.push(stargate),
            Err(error) => panic!("ERROR when retrieving value: {:#?}", error),
        }
    }
    stargates
}

#[cfg(test)]
mod tests {
    use crate::universe::stargate::{self, Stargate};

    #[test]
    fn get_ok() {
        let stargates: Vec<Stargate> = stargate::get(&30000001);
        let target: Vec<Stargate> = Vec::from([
            Stargate {
                destination_id: 30000003,
                destination_name: "Akpivem".to_string(),
            },
            Stargate {
                destination_id: 30000005,
                destination_name: "Sasta".to_string(),
            },
            Stargate {
                destination_id: 30000007,
                destination_name: "Yuzier".to_string(),
            },
        ]);
        assert_eq!(stargates, target)
    }

    #[test]
    fn get_none() {
        let stargates: Vec<Stargate> = stargate::get(&1321412421);
        assert_eq!(stargates.len(), 0);
    }
}
