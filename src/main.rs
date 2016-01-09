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

struct ScenHeader<'a> {
    version: &'a[u8; 4],
    header_type: i32,
    timestamp: i32,
    instructions: &'a str,
    players: Vec<Player<'a>>,
    filename: &'a str,
    messages: ScenMessages<'a>,
    image: ScenImage<'a>,
}

struct Player<'a> {
    name: &'a str,
    active: u32,
    human: u32,
    civilization: u32,
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
            try!(zlib_buf.write_u32::<LittleEndian>(self.players[i].active));
            try!(zlib_buf.write_u32::<LittleEndian>(self.players[i].human));
            try!(zlib_buf.write_u32::<LittleEndian>(self.players[i].civilization));
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
        for _ in 0..8 {
            try!(zlib_buf.write_u32::<LittleEndian>(100 /* gold */));
            try!(zlib_buf.write_u32::<LittleEndian>(200 /* wood */));
            try!(zlib_buf.write_u32::<LittleEndian>(200 /* food */));
            try!(zlib_buf.write_u32::<LittleEndian>(100 /* stone */));
            try!(zlib_buf.write_u32::<LittleEndian>(0 /* ore */));
            try!(zlib_buf.write_u32::<LittleEndian>(0 /* ??? */));
        }
        // Unused players
        try!(zlib_buf.write(&[0; 6 * 8 * 4]));

        // Separator
        try!(zlib_buf.write_u32::<LittleEndian>(0xFFFFFF9D));

        // Scenario goals: 10 * int32
        // Conquest; unknown; Relics; unknown; Exploration; unknown;
        // All; Mode; Score; Time Limit
        try!(zlib_buf.write(&[0; 10 * 4]));

        // Diplomacy
        for fromPlayer in 0..16 {
            for toPlayer in 0..16 {
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
        try!(zlib_buf.write_i32::<LittleEndian>(220 /* x */));
        try!(zlib_buf.write_i32::<LittleEndian>(220 /* y */));
        for x in 0..220 {
            for y in 0..220 {
                try!(zlib_buf.write(&[
                    ((x + y) % 40) as u8, // Type
                    1, // Elevation
                    0, // ???
                ]));
            }
        }

        // Units sections
        try!(zlib_buf.write_u32::<LittleEndian>(9));

        // Resources again??
        for i in 0..8 {
            try!(zlib_buf.write_f32::<LittleEndian>(100.0 /* gold */));
            try!(zlib_buf.write_f32::<LittleEndian>(200.0 /* wood */));
            try!(zlib_buf.write_f32::<LittleEndian>(200.0 /* food */));
            try!(zlib_buf.write_f32::<LittleEndian>(100.0 /* stone */));
            try!(zlib_buf.write_f32::<LittleEndian>(0.0 /* ore */));
            try!(zlib_buf.write_f32::<LittleEndian>(0.0 /* ??? */));
            try!(zlib_buf.write_f32::<LittleEndian>(0.0 /* population */));
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

        try!(zlib_buf.write_u32::<LittleEndian>(2576980378));
        try!(zlib_buf.write_u32::<LittleEndian>(1073322393));
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

        buf.write_all(&compressed_buf);
        Ok(buf)
    }
}

impl<'a> Player<'a> {
    fn empty<'b>() -> Player<'b> {
        Player { name: "", active: 0, human: 0, civilization: 0 }
    }
}

impl<'a> ScenMessages<'a> {
    fn to_bytes(&self) -> Result<Vec<u8>, io::Error> {
        let mut buf = vec![];
        for _ in 0..6 {
            try!(buf.write_i32::<LittleEndian>(0));
        }
        try!(buf.write_u16::<LittleEndian>(self.objectives.len() as u16));
        try!(buf.write(&self.objectives.as_bytes()));
        try!(buf.write_u16::<LittleEndian>(self.hints.len() as u16));
        try!(buf.write(&self.hints.as_bytes()));
        try!(buf.write_u16::<LittleEndian>(self.victory.len() as u16));
        try!(buf.write(&self.victory.as_bytes()));
        try!(buf.write_u16::<LittleEndian>(self.loss.len() as u16));
        try!(buf.write(&self.loss.as_bytes()));
        try!(buf.write_u16::<LittleEndian>(self.history.len() as u16));
        try!(buf.write(&self.history.as_bytes()));
        try!(buf.write_u16::<LittleEndian>(self.scouts.len() as u16));
        try!(buf.write(&self.scouts.as_bytes()));
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

fn test(filename: &str) -> Result<(), io::Error> {
    let mut buf = try!(File::create(filename));
    let header = ScenHeader {
        version: b"1.21",
        header_type: 2,
        timestamp: 1451422223,
        instructions: "Build a fancy-pants base!",
        filename: filename,
        players: vec![
            Player { name: "Hello World, from Rust!", active: 1, human: 2, civilization: 1 },
            Player { name: "Filthy Opponent", active: 1, human: 0, civilization: 18 },
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
        }
    };
    buf.write_all(&try!(header.to_bytes())).map(|_| ())
}

fn main() {
    match test("Test Scenario.scx") {
        Ok(()) => (),
        Err(e) => panic!("oops {}", e)
    }
}
