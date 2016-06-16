use std::io::{Write, Error};
use byteorder::{LittleEndian as LE, WriteBytesExt};

use consts::UnitType;

pub struct Unit {
    id: u32,
    unit_type: UnitType,
    x: f32,
    y: f32,
    angle: f32,
    frame: u16,
    garrison_id: i32,
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
        try!(buf.write_i32::<LE>(self.garrison_id));
        Ok(buf)
    }
}
