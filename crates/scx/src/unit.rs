use std::io::Error;
use byteorder::{LittleEndian as LE, WriteBytesExt};

use consts::UnitType;

pub struct Unit {
    pub id: u32,
    pub unit_type: UnitType,
    pub x: f32,
    pub y: f32,
    pub angle: f32,
    pub frame: u16,
    pub garrison_id: u32,
}

impl Unit {
    pub fn new(unit_type: UnitType, pos_x: f32, pos_y: f32) -> Unit {
        Unit {
            id: 1,
            unit_type: unit_type,
            x: pos_x,
            y: pos_y,
            angle: 0.0,
            frame: 0,
            garrison_id: 0,
        }
    }

    pub fn to_bytes(&self) -> Result<Vec<u8>, Error> {
        let mut buf = Vec::with_capacity(29);
        try!(buf.write_f32::<LE>(self.x));
        try!(buf.write_f32::<LE>(self.y));
        try!(buf.write_f32::<LE>(2.0));
        try!(buf.write_u32::<LE>(self.id));
        try!(buf.write_u16::<LE>(self.unit_type as u16));
        try!(buf.write_i8(2));
        try!(buf.write_f32::<LE>(self.angle));
        try!(buf.write_u16::<LE>(self.frame));
        try!(buf.write_u32::<LE>(self.garrison_id));
        Ok(buf)
    }
}
