use json::JsonValue;
use std::mem::transmute;

use scx;

pub trait JsonRevive {
    fn from_json(value: &JsonValue) -> Self;
}

impl JsonRevive for scx::Coordinate {
    fn from_json(json: &JsonValue) -> scx::Coordinate {
        scx::Coordinate::new(
            json["x"].as_u32().unwrap(),
            json["y"].as_u32().unwrap()
        )
    }
}

impl JsonRevive for scx::Rectangle {
    fn from_json(json: &JsonValue) -> scx::Rectangle {
        let x = json["x1"].as_u32().unwrap();
        let y = json["y1"].as_u32().unwrap();
        scx::Rectangle::new(
            x,
            y,
            json["x2"].as_u32().unwrap() - x,
            json["y2"].as_u32().unwrap() - y
        )
    }
}

impl JsonRevive for scx::MapTile {
    fn from_json(json: &JsonValue) -> scx::MapTile {
        scx::MapTile::new(json["t"].as_u8().unwrap(), json["e"].as_u8().unwrap())
    }
}
impl JsonRevive for scx::Map {
    fn from_json(json: &JsonValue) -> scx::Map {
        let mut map = scx::Map::new(match json["size"][0].as_u32() {
            Some(val) => val,
            None => panic!("Missing map size"),
        });
        for row in json["tiles"].members() {
            for cell in row.members() {
                map.tiles.push(scx::MapTile::from_json(cell));
            }
        }
        map
    }
}

impl JsonRevive for scx::TriggerCondition {
    fn from_json(json: &JsonValue) -> scx::TriggerCondition {
        scx::TriggerCondition {
            condition: json["type"].as_i32().unwrap(),
            check: 1,
            amount: json["amount"].as_i32().unwrap(),
            resource: json["resource"].as_i32().unwrap(),
            unit_object: json["object_source"].as_i32().unwrap(),
            unit_location: json["object_location"].as_i32().unwrap(),
            player: json["player"].as_i32().unwrap(),
            technology: json["technology"].as_i32().unwrap(),
            timer: json["time"].as_i32().unwrap(),
            area: scx::Rectangle::from_json(&json["area"]),
            unit_group: json["unit_group"].as_i32().unwrap(),
            unit_type: json["unit_type"].as_i32().unwrap(),
            ai_signal: json["ai_signal"].as_i32().unwrap(),
        }
    }
}

impl JsonRevive for scx::Unit {
    fn from_json(json: &JsonValue) -> scx::Unit {
        scx::Unit {
            id: json["id"].as_u32().unwrap(),
            unit_type: unsafe { transmute( json["type"].as_u16().unwrap() ) },
            x: json["x"].as_f32().unwrap(),
            y: json["y"].as_f32().unwrap(),
            angle: json["angle"].as_f32().unwrap(),
            frame: json["frame"].as_u16().unwrap(),
            garrison_id: json["garrison_id"].as_u32().unwrap(),
        }
    }
}

impl JsonRevive for scx::Player {
    fn from_json(json: &JsonValue) -> scx::Player {
        scx::Player {
            name: String::from(json["name"].as_str().unwrap()),
            active: 1,
            human: 2,
            civilization: unsafe { transmute(json["civilization"].as_u8().unwrap()) },
            resources: scx::BaseResources::default(),
            units: vec![],
        }
    }
}
