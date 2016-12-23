use std::io;
use std::io::prelude::*;
use std::mem;
use byteorder::{LittleEndian as LE, WriteBytesExt};
use flate2::{Compression, Flush, Compress, Status};

use map::Map;
use player::Player;

const HEADER_SEPARATOR: u32 = 0xFFFFFF9D;

pub struct ScenHeader<'a> {
    pub version: &'a[u8; 4],
    pub header_type: i32,
    pub timestamp: i32,
    pub instructions: &'a str,
    pub players: Vec<Player<'a>>,
    pub filename: &'a str,
    pub messages: ScenMessages<'a>,
    pub image: ScenImage<'a>,
    pub map: Map,
}

pub struct ScenMessages<'a> {
    pub objectives: &'a str,
    pub hints: &'a str,
    pub scouts: &'a str,
    pub history: &'a str,
    pub victory: &'a str,
    pub loss: &'a str,
}

pub struct ScenImage<'a> {
    pub filename: &'a str,
    pub included: bool,
    pub width: i32,
    pub height: i32,
    pub include: i16,
}

impl<'a> ScenHeader<'a> {
    pub fn to_bytes(&self) -> Result<Vec<u8>, io::Error> {
        let mut buf = vec![];

        let instructions_length = self.instructions.len() as i32;
        let header_length = 20 + instructions_length;
        try!(buf.write(self.version));
        try!(buf.write_i32::<LE>(header_length));
        try!(buf.write_i32::<LE>(self.header_type));
        try!(buf.write_i32::<LE>(self.timestamp));
        try!(buf.write_i32::<LE>(instructions_length));
        try!(buf.write(self.instructions.as_bytes()));
        try!(buf.write_i32::<LE>(0));
        try!(buf.write_i32::<LE>(self.players.len() as i32));

        let mut zlib_buf = vec![];
        try!(zlib_buf.write_u32::<LE>(19246));
        try!(zlib_buf.write_f32::<LE>(1.22 /* UserPatch */));
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
                try!(zlib_buf.write_i32::<LE>(0));
            } else {
                try!(zlib_buf.write_i32::<LE>(0));
            }
        }

        for i in 0..16 {
            if self.players.len() <= i {
                try!(zlib_buf.write_u32::<LE>(0));
                try!(zlib_buf.write_u32::<LE>(0));
                try!(zlib_buf.write_u32::<LE>(0));
                try!(zlib_buf.write_u32::<LE>(4));
                continue;
            }
            let ref p = self.players[i];
            try!(zlib_buf.write_u32::<LE>(p.active));
            try!(zlib_buf.write_u32::<LE>(p.human));
            try!(zlib_buf.write_u32::<LE>(p.civilization as u32));
            try!(zlib_buf.write_u32::<LE>(4));
        }

        try!(zlib_buf.write_u32::<LE>(1));
        try!(zlib_buf.write_all(&[0]));
        try!(zlib_buf.write_f32::<LE>(-1.0));
        try!(zlib_buf.write_u16::<LE>(self.filename.len() as u16));
        try!(zlib_buf.write_all(self.filename.as_bytes()));

        try!(zlib_buf.write_all(
            &try!(self.messages.to_bytes())
        ));

        // cinematics
        try!(zlib_buf.write_u16::<LE>(0));
        try!(zlib_buf.write_u16::<LE>(0));
        try!(zlib_buf.write_u16::<LE>(0));

        try!(zlib_buf.write_all(
            &try!(self.image.to_bytes())
        ));

        for _ in 0..16 {
            // two 0-length strings
            try!(zlib_buf.write_u16::<LE>(0));
            try!(zlib_buf.write_u16::<LE>(0));
        }

        // Player AI names
        for _ in 0..8 {
            try!(zlib_buf.write_u16::<LE>(0));
        }
        // Unused players
        for _ in 0..8 {
            let name = "RandomGame";
            try!(zlib_buf.write_u16::<LE>(name.len() as u16));
            try!(zlib_buf.write(&name.as_bytes()));
        }
        // AI source code
        for _ in 0..8 {
            try!(zlib_buf.write_i32::<LE>(0));
            try!(zlib_buf.write_i32::<LE>(0));
            // 0-length AI source code string
            try!(zlib_buf.write_i32::<LE>(0));
        }
        // Source code for unused players
        for _ in 0..(3 * 8) {
            try!(zlib_buf.write_i32::<LE>(0));
        }

        // AI type
        for _ in 0..8 {
            try!(zlib_buf.write(&[0]));
        }
        // Unused players
        try!(zlib_buf.write(&[0; 8]));

        try!(zlib_buf.write_u32::<LE>(HEADER_SEPARATOR));

        // Resources
        for i in 0..16 {
            if self.players.len() > i {
                let p = &self.players[i];
                try!(zlib_buf.write_u32::<LE>(p.resources.food));
                try!(zlib_buf.write_u32::<LE>(p.resources.wood));
                try!(zlib_buf.write_u32::<LE>(p.resources.gold));
                try!(zlib_buf.write_u32::<LE>(p.resources.stone));
                try!(zlib_buf.write_u32::<LE>(p.resources.ore));
                try!(zlib_buf.write_u32::<LE>(0 /* ??? */));
            }
            else {
                // Unused players
                try!(zlib_buf.write(&vec![0; 6 * mem::size_of::<u32>()]));
            }
        }

        try!(zlib_buf.write_u32::<LE>(HEADER_SEPARATOR));

        // Scenario goals: 10 * int32
        // Conquest; unknown; Relics; unknown; Exploration; unknown;
        // All; Mode; Score; Time Limit
        let scenario_goals_size = 10 * mem::size_of::<i32>();
        try!(zlib_buf.write(&vec![0; scenario_goals_size]));

        // Diplomacy
        for _ in 0..16 {
            for _ in 0..16 {
                try!(zlib_buf.write_i32::<LE>(0));
            }
        }

        // ???
        try!(zlib_buf.write(&[0; 11520]));

        try!(zlib_buf.write_u32::<LE>(HEADER_SEPARATOR));

        // Allied victory
        for _ in 0..16 {
            try!(zlib_buf.write_i32::<LE>(0));
        }

        // Technology count??
        for _ in 0..8 {
            try!(zlib_buf.write_i32::<LE>(0));
        }
        for _ in 0..8 {
            try!(zlib_buf.write_i32::<LE>(-1));
        }
        // Technology something??
        for _ in 0..(16 * 30) {
            try!(zlib_buf.write_i32::<LE>(-1));
        }

        // Unit count??
        for _ in 0..8 {
            try!(zlib_buf.write_i32::<LE>(0));
        }
        for _ in 0..8 {
            try!(zlib_buf.write_i32::<LE>(-1));
        }
        // Unit something??
        for _ in 0..(16 * 30) {
            try!(zlib_buf.write_i32::<LE>(-1));
        }

        // Building count??
        for _ in 0..8 {
            try!(zlib_buf.write_i32::<LE>(0));
        }
        for _ in 0..8 {
            try!(zlib_buf.write_i32::<LE>(-1));
        }
        // Buildings something??
        for _ in 0..(16 * 20) {
            try!(zlib_buf.write_i32::<LE>(-1));
        }

        // ???
        try!(zlib_buf.write_u32::<LE>(0));
        try!(zlib_buf.write_u32::<LE>(0));
        // All Techs
        try!(zlib_buf.write_u32::<LE>(0));

        // Starting age
        for _ in 0..8 {
            try!(zlib_buf.write_u32::<LE>(0));
        }
        // Gaia
        try!(zlib_buf.write_u32::<LE>(0));
        // Unused
        for _ in 1..8 {
            try!(zlib_buf.write_i32::<LE>(-1));
        }

        try!(zlib_buf.write_u32::<LE>(HEADER_SEPARATOR));

        // Camera
        try!(zlib_buf.write_i32::<LE>(0 /* x */));
        try!(zlib_buf.write_i32::<LE>(0 /* y */));

        // AI type
        try!(zlib_buf.write_u32::<LE>(0));

        // Map tiles
        try!(zlib_buf.write(
            &try!(self.map.to_bytes())
        ));

        // Units sections
        try!(zlib_buf.write_u32::<LE>(9));

        // Resources again??
        for i in 0..8 {
            if self.players.len() > i {
                let p = &self.players[i];
                try!(zlib_buf.write_f32::<LE>(p.resources.food as f32));
                try!(zlib_buf.write_f32::<LE>(p.resources.wood as f32));
                try!(zlib_buf.write_f32::<LE>(p.resources.gold as f32));
                try!(zlib_buf.write_f32::<LE>(p.resources.stone as f32));
                try!(zlib_buf.write_f32::<LE>(p.resources.ore as f32));
                try!(zlib_buf.write_f32::<LE>(0.0 /* ??? */));
                try!(zlib_buf.write_f32::<LE>(0.0 /* population */));
            } else {
                // Unused players
                try!(zlib_buf.write(&vec![0; 7 * mem::size_of::<f32>()]));
            }
        }

        for i in 0..9 {
            // Zero units
            if i > 0 && self.players.len() >= i {
                let units = &self.players[i - 1].units;
                println!("Units: p{} {}", i, units.len());
                try!(zlib_buf.write_u32::<LE>(units.len() as u32));
                for unit in units {
                    try!(zlib_buf.write(
                        &try!(unit.to_bytes())
                    ));
                }
            } else {
                try!(zlib_buf.write_u32::<LE>(0));
            }
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
        try!(zlib_buf.write_u32::<LE>(9));

        for player in 1..9 {
            let name = "Promisory";
            try!(zlib_buf.write_i16::<LE>(name.len() as i16));
            try!(zlib_buf.write(&name.as_bytes()));
            try!(zlib_buf.write_f32::<LE>(101.0 /* camera x */));
            try!(zlib_buf.write_f32::<LE>(101.0 /* camera y */));
            try!(zlib_buf.write_i16::<LE>(101 /* ?? */));
            try!(zlib_buf.write_i16::<LE>(101 /* ?? */));
            try!(zlib_buf.write(&[0])); // allied victory (again?)
            // Diplomacy again
            try!(zlib_buf.write_u16::<LE>(9));
            try!(zlib_buf.write(&[0; 9]));
            // Diplo to gaia?? from gaia?
            for _ in 0..9 {
                try!(zlib_buf.write_i32::<LE>(0));
            }
            // Player colour
            try!(zlib_buf.write_u32::<LE>(player));
            // ???
            try!(zlib_buf.write_f32::<LE>(2.0));
            try!(zlib_buf.write_u16::<LE>(0));
            // ???
            try!(zlib_buf.write(&[0; 8 + 7]));
            try!(zlib_buf.write_i32::<LE>(-1));
        }

        try!(zlib_buf.write_u32::<LE>(0x9999999A));
        try!(zlib_buf.write_u32::<LE>(0x3FF99999));
        try!(zlib_buf.write(&[0]));
        // Triggers
        try!(zlib_buf.write_i32::<LE>(0));

        try!(zlib_buf.write_u32::<LE>(0));
        try!(zlib_buf.write_u32::<LE>(0));

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

impl<'a> ScenMessages<'a> {
    fn message_to_bytes(buf: &mut Vec<u8>, message: &str) -> Result<(), io::Error> {
        try!(buf.write_u16::<LE>(1 + (message.len() as u16)));
        try!(buf.write(&message.as_bytes()));
        try!(buf.write_u8(0));
        Ok(())
    }

    fn to_bytes(&self) -> Result<Vec<u8>, io::Error> {
        let mut buf = vec![];
        // String table indices
        for _ in 0..6 {
            try!(buf.write_i32::<LE>(0));
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

impl<'a> ScenImage<'a> {
    fn to_bytes(&self) -> Result<Vec<u8>, io::Error> {
        let mut buf = vec![];
        try!(buf.write_u16::<LE>(self.filename.len() as u16));
        try!(buf.write(&self.filename.as_bytes()));
        try!(buf.write_i32::<LE>(if self.included { 1 } else { 0 }));
        try!(buf.write_i32::<LE>(self.width));
        try!(buf.write_i32::<LE>(self.height));
        try!(buf.write_i16::<LE>(self.include));
        Ok(buf)
    }
}
