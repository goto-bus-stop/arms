extern crate hlua;
extern crate json;
extern crate arms_scx as scx;

mod json_revive;
mod scripting;

use std::io;
use std::io::prelude::*;
use std::fs::File;
use hlua::LuaError;

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

fn test(filename: &str) -> Result<(), io::Error> {
    let mut f = try!(File::open("Scenario.lua"));
    let mut s = String::new();
    try!(f.read_to_string(&mut s));
    let map = match scripting::run_lua(&s) {
        Ok(result) => match json::parse(&result) {
            Ok(mut json) => Map::from_json(json["map"].take()),
            Err(_) => panic!("Json error"),
        },
        Err(LuaError::SyntaxError(message)) => panic!(message),
        Err(LuaError::ExecutionError(message)) => panic!(message),
        Err(LuaError::ReadError(_)) => panic!("hlua io error"),
        Err(LuaError::WrongType) => panic!("wrong type"),
    };

    let mut buf = try!(File::create(filename));

    let header = ScenHeader {
        version: b"1.21",
        header_type: 2,
        timestamp: 1451422223,
        instructions: "Build a fancy-pants base!",
        filename: filename,
        players: vec![
            Player {
                name: "Hello World, from Rust!",
                active: 1,
                human: 2,
                civilization: Civilization::Britons,
                resources: BaseResources {
                    wood: 100,
                    food: 200,
                    gold: 300,
                    stone: 400,
                    ore: 0,
                },
                units: vec![
                    Unit::new(UnitType::ScoutCavalry, (map.size as f32) / 2.0, (map.size as f32) / 2.0),
                    Unit::new(UnitType::ScoutCavalry, (map.size as f32) / 2.0 + 5.0, (map.size as f32) / 2.0 + 5.0),
                ],
            },
            Player {
                name: "Filthy Opponent",
                active: 1,
                human: 0,
                civilization: Civilization::Koreans,
                resources: BaseResources {
                    wood: 200,
                    food: 200,
                    gold: 100,
                    stone: 200,
                    ore: 0,
                },
                units: vec![],
            },
        ],
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
