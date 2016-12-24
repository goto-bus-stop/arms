extern crate lua;
extern crate json;
extern crate arms_scx as scx;

mod json_revive;
mod scripting;

use std::io;
use std::io::prelude::*;
use std::fs::File;
use std::collections::HashMap;

use json_revive::JsonRevive;
use scx::{
    Civilization,
    UnitType,
    Map,
    BaseResources,
    Player,
    Unit,
    ScenHeader,
    ScenMessages,
    ScenImage
};

struct PlayerWithNumber {
    number: i8,
    player: scx::Player
}

fn test(filename: &str) -> Result<(), io::Error> {
    let mut f = try!(File::open("Scenario.lua"));
    let mut s = String::new();
    try!(f.read_to_string(&mut s));
    let result = try!(scripting::run_lua(&s));
    let mut tree = match json::parse(&result) {
        Ok(tree) => tree,
        Err(_) => panic!("Json Error")
    };
    let map = Map::from_json(&tree["map"]);

    let mut units: HashMap<i8, Vec<Unit>> = HashMap::new();
    for player in tree["players"].members() {
        units.insert(player["number"].as_i8().unwrap(), Vec::new());
    }

    for unit in tree["units"].members() {
        let number = unit["owner"].as_i8().unwrap();
        match units.get_mut(&number) {
            Some(player) => player.push(Unit::from_json(unit)),
            None => (),
        };
    }

    let mut players = tree["players"].members()
        .map(|player| {
            let number = player["number"].as_i8().unwrap();
            let instance = scx::Player::from_json(player);
            match units.remove(&number) {
                Some(player_units) => instance.with_units(player_units),
                None => instance
            }
        });

    let mut buf = try!(File::create(filename));

    let header = ScenHeader {
        version: b"1.21",
        header_type: 2,
        timestamp: 1451422223,
        instructions: "Build a fancy-pants base!",
        filename: filename,
        players: players.collect(),
        messages: ScenMessages {
            objectives: "",
            hints: "",
            scouts: "",
            history: "",
            victory: "",
            loss: "",
        },
        image: ScenImage {
            filename: "",
            included: false,
            width: 0,
            height: 0,
            include: 1,
        },
        map: map,
    };

    buf.write_all(&try!(header.to_bytes())).map(|_| ())
}

fn main() {
    match test("Test Scenario.scx") {
        Ok(()) => (),
        Err(e) => panic!("oops {}", e)
    }
}
