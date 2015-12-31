extern crate byteorder;
extern crate flate2;

use std::io;
use std::io::prelude::*;
use std::fs::File;
use std::mem;
use byteorder::{LittleEndian, WriteBytesExt};
use flate2::Compression;
use flate2::write::DeflateEncoder;

struct ScenHeader<'a> {
    version: &'a[u8; 4],
    header_type: i32,
    timestamp: i32,
    instructions: &'a str,
    players: Vec<Player<'a>>,
    filename: &'a str,
    messages: ScenMessages<'a>
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

        let mut zlib_buf = DeflateEncoder::new(buf, Compression::Default);
        try!(zlib_buf.write_u32::<LittleEndian>(0));
        try!(zlib_buf.write_f32::<LittleEndian>(1.22 /* UserPatch */));
        for i in 0..8 {
            if self.players.len() > i {
                let name = self.players[i].name;
                try!(zlib_buf.write_all(name.as_bytes()));
                try!(zlib_buf.write_all(&vec![0; 256 - name.len()]));
            } else {
                try!(zlib_buf.write_all(&vec![0; 256]));
            }
        }
        try!(zlib_buf.write_all(&vec![0; 8 * 256]));

        for i in 0..8 {
            if self.players.len() > i {
                // player name ID in string table
                try!(zlib_buf.write_i32::<LittleEndian>(0));
            } else {
                try!(zlib_buf.write_i32::<LittleEndian>(0));
            }
        }
        for _ in 0..8 {
            try!(zlib_buf.write_i32::<LittleEndian>(-1));
        }

        for i in 0..8 {
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

        // 8 * 4 times uint 0
        try!(zlib_buf.write_all(&vec![0; 8 * 4 * mem::size_of::<u32>()]));

        try!(zlib_buf.write_u32::<LittleEndian>(0));
        try!(zlib_buf.write_f32::<LittleEndian>(0.0));
        try!(zlib_buf.write_all(&[46]));
        try!(zlib_buf.write_all(self.filename.as_bytes()));
        try!(zlib_buf.write_all(
            &try!(self.messages.to_bytes())
        ));

        zlib_buf.finish()
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
        for _ in 1..6 {
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

fn test(filename: &str) -> Result<(), io::Error> {
    let mut buf = try!(File::create(filename));
    let header = ScenHeader {
        version: b"1.21",
        header_type: 2,
        timestamp: 1451422223,
        instructions: "Build a fancy-pants base!",
        filename: filename,
        players: vec![
            Player { name: "William Wallace", active: 1, human: 2, civilization: 1 },
            Player { name: "Sejong the Great", active: 0, human: 2, civilization: 18 },
        ],
        messages: ScenMessages {
            objectives: "",
            hints: "",
            scouts: "",
            history: "",
            victory: "",
            loss: "",
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
