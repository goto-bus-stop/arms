extern crate byteorder;
extern crate flate2;

use std::io;
use std::io::prelude::*;
use std::fs::File;
use std::mem;
use byteorder::{LittleEndian, WriteBytesExt};
use flate2::Compression;
use flate2::Flush;
use flate2::Compress;
use flate2::Status;

mod consts;

use consts::{Civilization};

struct ScenHeader<'a> {
    version: &'a[u8; 4],
    header_type: i32,
    timestamp: i32,
    instructions: &'a str,
    players: Vec<Player<'a>>,
    filename: &'a str,
    messages: ScenMessages<'a>,
    image: ScenImage<'a>,
    map: Map,
}

struct Player<'a> {
    name: &'a str,
    active: u32,
    human: u32,
    civilization: Civilization,
    resources: BaseResources
}

struct ScenMessages<'a> {
    objectives: &'a str,
    hints: &'a str,
    scouts: &'a str,
    history: &'a str,
    victory: &'a str,
    loss: &'a str,
}

struct ScenImage<'a> {
    filename: &'a str,
    included: bool,
    width: i32,
    height: i32,
    include: i16
}

struct BaseResources {
    gold: u32,
    wood: u32,
    food: u32,
    stone: u32,
    ore: u32,
}

struct MapTile {
    terrain: u8,
    elevation: u8,
}

struct Map {
    size: u32,
    tiles: Vec<MapTile>
}

impl<'a> ScenHeader<'a> {
    fn to_bytes(&self) -> Result<Vec<u8>, io::Error> {
        let mut buf = vec![];

        let instructions_length = self.instructions.len() as i32;
        let header_length = 20 + instructions_length;
        try!(buf.write(self.version));
        try!(buf.write_i32::<LittleEndian>(header_length));
        try!(buf.write_i32::<LittleEndian>(self.header_type));
        try!(buf.write_i32::<LittleEndian>(self.timestamp));
        try!(buf.write_i32::<LittleEndian>(instructions_length));
        try!(buf.write(self.instructions.as_bytes()));
        try!(buf.write_i32::<LittleEndian>(0));
        try!(buf.write_i32::<LittleEndian>(self.players.len() as i32));

        let mut zlib_buf = vec![];
        try!(zlib_buf.write_u32::<LittleEndian>(19246));
        try!(zlib_buf.write_f32::<LittleEndian>(1.22 /* UserPatch */));
        for i in 0..16 {
            if self.players.len() > i {
                let name = self.players[i].name;
                try!(zlib_buf.write_all(name.as_bytes()));
                try!(zlib_buf.write_all(&vec![0; 256 - name.len()]));
            } else {
                try!(zlib_buf.write_all(&vec![0; 256]));
            }
        }

        for i in 0..16 {
            if self.players.len() > i {
                // player name ID in string table
                try!(zlib_buf.write_i32::<LittleEndian>(0));
            } else {
                try!(zlib_buf.write_i32::<LittleEndian>(0));
            }
        }

        for i in 0..16 {
            if self.players.len() <= i {
                try!(zlib_buf.write_u32::<LittleEndian>(0));
                try!(zlib_buf.write_u32::<LittleEndian>(0));
                try!(zlib_buf.write_u32::<LittleEndian>(0));
                try!(zlib_buf.write_u32::<LittleEndian>(4));
                continue;
            }
            let ref p = self.players[i];
            try!(zlib_buf.write_u32::<LittleEndian>(p.active));
            try!(zlib_buf.write_u32::<LittleEndian>(p.human));
            try!(zlib_buf.write_u32::<LittleEndian>(p.civilization as u32));
            try!(zlib_buf.write_u32::<LittleEndian>(4));
        }

        try!(zlib_buf.write_u32::<LittleEndian>(1));
        try!(zlib_buf.write_all(&[0]));
        try!(zlib_buf.write_f32::<LittleEndian>(-1.0));
        try!(zlib_buf.write_u16::<LittleEndian>(self.filename.len() as u16));
        try!(zlib_buf.write_all(self.filename.as_bytes()));

        try!(zlib_buf.write_all(
            &try!(self.messages.to_bytes())
        ));

        // cinematics
        try!(zlib_buf.write_u16::<LittleEndian>(0));
        try!(zlib_buf.write_u16::<LittleEndian>(0));
        try!(zlib_buf.write_u16::<LittleEndian>(0));

        try!(zlib_buf.write_all(
            &try!(self.image.to_bytes())
        ));

        for _ in 0..16 {
            // two 0-length strings
            try!(zlib_buf.write_u16::<LittleEndian>(0));
            try!(zlib_buf.write_u16::<LittleEndian>(0));
        }

        // Player AI names
        for _ in 0..8 {
            try!(zlib_buf.write_u16::<LittleEndian>(0));
        }
        // Unused players
        for _ in 0..8 {
            let name = "RandomGame";
            try!(zlib_buf.write_u16::<LittleEndian>(name.len() as u16));
            try!(zlib_buf.write(&name.as_bytes()));
        }
        // AI source code
        for _ in 0..8 {
            try!(zlib_buf.write_i32::<LittleEndian>(0));
            try!(zlib_buf.write_i32::<LittleEndian>(0));
            // 0-length AI source code string
            try!(zlib_buf.write_i32::<LittleEndian>(0));
        }
        // Source code for unused players
        for _ in 0..(3 * 8) {
            try!(zlib_buf.write_i32::<LittleEndian>(0));
        }

        // AI type
        for _ in 0..8 {
            try!(zlib_buf.write(&[0]));
        }
        // Unused players
        try!(zlib_buf.write(&[0; 8]));

        // Separator
        try!(zlib_buf.write_u32::<LittleEndian>(0xFFFFFF9D));

        // Resources
        for i in 0..16 {
            if self.players.len() > i {
                let p = &self.players[i];
                try!(zlib_buf.write_u32::<LittleEndian>(p.resources.food));
                try!(zlib_buf.write_u32::<LittleEndian>(p.resources.wood));
                try!(zlib_buf.write_u32::<LittleEndian>(p.resources.gold));
                try!(zlib_buf.write_u32::<LittleEndian>(p.resources.stone));
                try!(zlib_buf.write_u32::<LittleEndian>(p.resources.ore));
                try!(zlib_buf.write_u32::<LittleEndian>(0 /* ??? */));
            }
            else {
                // Unused players
                try!(zlib_buf.write(&vec![0; 6 * mem::size_of::<u32>()]));
            }
        }

        // Separator
        try!(zlib_buf.write_u32::<LittleEndian>(0xFFFFFF9D));

        // Scenario goals: 10 * int32
        // Conquest; unknown; Relics; unknown; Exploration; unknown;
        // All; Mode; Score; Time Limit
        let scenario_goals_size = 10 * mem::size_of::<i32>();
        try!(zlib_buf.write(&vec![0; scenario_goals_size]));

        // Diplomacy
        for from_player in 0..16 {
            for to_player in 0..16 {
                try!(zlib_buf.write_i32::<LittleEndian>(0));
            }
        }

        // ???
        try!(zlib_buf.write(&[0; 11520]));

        // Separator
        try!(zlib_buf.write_u32::<LittleEndian>(0xFFFFFF9D));

        // Allied victory
        for player in 0..16 {
            try!(zlib_buf.write_i32::<LittleEndian>(0));
        }

        // Technology count??
        for player in 0..8 {
            try!(zlib_buf.write_i32::<LittleEndian>(0));
        }
        for player in 0..8 {
            try!(zlib_buf.write_i32::<LittleEndian>(-1));
        }
        // Technology something??
        for _ in 0..(16 * 30) {
            try!(zlib_buf.write_i32::<LittleEndian>(-1));
        }

        // Unit count??
        for player in 0..8 {
            try!(zlib_buf.write_i32::<LittleEndian>(0));
        }
        for player in 0..8 {
            try!(zlib_buf.write_i32::<LittleEndian>(-1));
        }
        // Unit something??
        for _ in 0..(16 * 30) {
            try!(zlib_buf.write_i32::<LittleEndian>(-1));
        }

        // Building count??
        for player in 0..8 {
            try!(zlib_buf.write_i32::<LittleEndian>(0));
        }
        for player in 0..8 {
            try!(zlib_buf.write_i32::<LittleEndian>(-1));
        }
        // Buildings something??
        for _ in 0..(16 * 20) {
            try!(zlib_buf.write_i32::<LittleEndian>(-1));
        }

        // ???
        try!(zlib_buf.write_u32::<LittleEndian>(0));
        try!(zlib_buf.write_u32::<LittleEndian>(0));
        // All Techs
        try!(zlib_buf.write_u32::<LittleEndian>(0));

        // Starting age
        for _ in 0..8 {
            try!(zlib_buf.write_u32::<LittleEndian>(0));
        }
        // Gaia
        try!(zlib_buf.write_u32::<LittleEndian>(0));
        // Unused
        for _ in 1..8 {
            try!(zlib_buf.write_i32::<LittleEndian>(-1));
        }

        // Separator
        try!(zlib_buf.write_u32::<LittleEndian>(0xFFFFFF9D));

        // Camera
        try!(zlib_buf.write_i32::<LittleEndian>(0 /* x */));
        try!(zlib_buf.write_i32::<LittleEndian>(0 /* y */));

        // AI type
        try!(zlib_buf.write_u32::<LittleEndian>(0));

        // Map tiles
        try!(zlib_buf.write(
            &try!(self.map.to_bytes())
        ));

        // Units sections
        try!(zlib_buf.write_u32::<LittleEndian>(9));

        // Resources again??
        for i in 0..8 {
            if self.players.len() > i {
                let p = &self.players[i];
                try!(zlib_buf.write_f32::<LittleEndian>(p.resources.food as f32));
                try!(zlib_buf.write_f32::<LittleEndian>(p.resources.wood as f32));
                try!(zlib_buf.write_f32::<LittleEndian>(p.resources.gold as f32));
                try!(zlib_buf.write_f32::<LittleEndian>(p.resources.stone as f32));
                try!(zlib_buf.write_f32::<LittleEndian>(p.resources.ore as f32));
                try!(zlib_buf.write_f32::<LittleEndian>(0.0 /* ??? */));
                try!(zlib_buf.write_f32::<LittleEndian>(0.0 /* population */));
            }
            else {
                // Unused players
                try!(zlib_buf.write(&vec![0; 7 * mem::size_of::<f32>()]));
            }
        }

        for i in 0..9 {
            // Zero units
            try!(zlib_buf.write_u32::<LittleEndian>(0));
            // for unit in players[i].units:
            //     putFloat(unit.x)
            //     putFloat(unit.y)
            //     putFloat(unit.unknown1)
            //     putUInt32(unit.id)
            //     putUInt16(unit.type)
            //     putInt8(unit.unknown2)
            //     putFloat(unit.angle)
            //     putUInt16(unit.frame)
            //     putInt32(unit.inId)
        }

        // Playable players
        try!(zlib_buf.write_u32::<LittleEndian>(9));

        for player in 1..9 {
            let name = "Promisory";
            try!(zlib_buf.write_i16::<LittleEndian>(name.len() as i16));
            try!(zlib_buf.write(&name.as_bytes()));
            try!(zlib_buf.write_f32::<LittleEndian>(101.0 /* camera x */));
            try!(zlib_buf.write_f32::<LittleEndian>(101.0 /* camera y */));
            try!(zlib_buf.write_i16::<LittleEndian>(101 /* ?? */));
            try!(zlib_buf.write_i16::<LittleEndian>(101 /* ?? */));
            try!(zlib_buf.write(&[0])); // allied victory (again?)
            // Diplomacy again
            try!(zlib_buf.write_u16::<LittleEndian>(9));
            try!(zlib_buf.write(&[0; 9]));
            // Diplo to gaia?? from gaia?
            for _ in 0..9 {
                try!(zlib_buf.write_i32::<LittleEndian>(0));
            }
            // Player colour
            try!(zlib_buf.write_u32::<LittleEndian>(player));
            // ???
            try!(zlib_buf.write_f32::<LittleEndian>(2.0));
            try!(zlib_buf.write_u16::<LittleEndian>(0));
            // ???
            try!(zlib_buf.write(&[0; 8 + 7]));
            try!(zlib_buf.write_i32::<LittleEndian>(-1));
        }

        try!(zlib_buf.write_u32::<LittleEndian>(0x9999999A));
        try!(zlib_buf.write_u32::<LittleEndian>(0x3FF99999));
        try!(zlib_buf.write(&[0]));
        // Triggers
        try!(zlib_buf.write_i32::<LittleEndian>(0));

        try!(zlib_buf.write_u32::<LittleEndian>(0));
        try!(zlib_buf.write_u32::<LittleEndian>(0));

        let mut compressed_buf = vec![];
        compressed_buf.reserve(zlib_buf.len());
        let mut compressor = Compress::new(Compression::Default, false);
        match compressor.compress_vec(&zlib_buf, &mut compressed_buf, Flush::Sync) {
            Status::Ok => println!("hoi"),
            Status::BufError => panic!("BufError"),
            Status::StreamEnd => panic!("StreamEnd"),
        };

        println!("compressed: {} -> {}", zlib_buf.len(), compressed_buf.len());

        try!(buf.write_all(&compressed_buf));
        Ok(buf)
    }
}

impl<'a> Player<'a> {
    fn empty<'b>() -> Player<'b> {
        Player {
            name: "",
            active: 0,
            human: 0,
            civilization: Civilization::None,
            resources: BaseResources {
                wood: 0,
                food: 0,
                gold: 0,
                stone: 0,
                ore: 0,
            },
        }
    }
}

impl<'a> ScenMessages<'a> {
    fn message_to_bytes(buf: &mut Vec<u8>, message: &str) -> Result<(), io::Error> {
        try!(buf.write_u16::<LittleEndian>(1 + (message.len() as u16)));
        try!(buf.write(&message.as_bytes()));
        try!(buf.write_u8(0));
        Ok(())
    }

    fn to_bytes(&self) -> Result<Vec<u8>, io::Error> {
        let mut buf = vec![];
        for _ in 0..6 {
            try!(buf.write_i32::<LittleEndian>(0));
        }
        try!(ScenMessages::message_to_bytes(&mut buf, &self.objectives));
        try!(ScenMessages::message_to_bytes(&mut buf, &self.hints));
        try!(ScenMessages::message_to_bytes(&mut buf, &self.victory));
        try!(ScenMessages::message_to_bytes(&mut buf, &self.loss));
        try!(ScenMessages::message_to_bytes(&mut buf, &self.history));
        try!(ScenMessages::message_to_bytes(&mut buf, &self.scouts));
        Ok(buf)
    }
}

impl <'a> ScenImage<'a> {
    fn to_bytes(&self) -> Result<Vec<u8>, io::Error> {
        let mut buf = vec![];
        try!(buf.write_u16::<LittleEndian>(self.filename.len() as u16));
        try!(buf.write(&self.filename.as_bytes()));
        try!(buf.write_i32::<LittleEndian>(if self.included { 1 } else { 0 }));
        try!(buf.write_i32::<LittleEndian>(self.width));
        try!(buf.write_i32::<LittleEndian>(self.height));
        try!(buf.write_i16::<LittleEndian>(self.include));
        Ok(buf)
    }
}

impl MapTile {
    fn new(terrain: u8, elevation: u8) -> MapTile {
        MapTile {
            terrain: terrain,
            elevation: elevation,
        }
    }

    fn to_bytes(&self) -> Result<[u8; 3], io::Error> {
        Ok([ self.terrain, self.elevation, 0 ])
    }
}

impl Map {
    fn new(size: u32) -> Map {
        Map {
            size: size,
            tiles: Vec::with_capacity((size * size) as usize)
        }
    }

    fn tile_at(&self, x: u32, y: u32) -> Option<&MapTile> {
        let idx = (self.size * y + x) as usize;
        if self.tiles.len() > idx {
            Some(&self.tiles[idx])
        } else {
            None
        }
    }

    fn to_bytes(&self) -> Result<Vec<u8>, io::Error> {
        let size = self.size as usize;
        let mut buf = Vec::with_capacity(
            size * size * mem::size_of::<MapTile>() +
            2 * mem::size_of::<u32>()
        );
        try!(buf.write_u32::<LittleEndian>(self.size));
        try!(buf.write_u32::<LittleEndian>(self.size));
        for x in 0..self.size {
            for y in 0..self.size {
                try!(buf.write(
                    &try!(self.tile_at(x, y).unwrap().to_bytes())
                ));
            }
        }
        Ok(buf)
    }
}

fn test(filename: &str) -> Result<(), io::Error> {
    let mut buf = try!(File::create(filename));
    let mut map = Map::new(220);

    for i in 0..map.tiles.capacity() {
        map.tiles.push(MapTile::new(((i / 7) % 7) as u8, 1));
    }

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
