use std::io::{Write, Error};
use std::mem;
use byteorder::{LittleEndian as LE, WriteBytesExt};

pub struct MapTile {
    terrain: u8,
    elevation: u8,
}

pub struct Map {
    pub size: u32,
    pub tiles: Vec<MapTile>,
}

impl MapTile {
    pub fn new(terrain: u8, elevation: u8) -> MapTile {
        MapTile {
            terrain: terrain,
            elevation: elevation,
        }
    }

    pub fn to_bytes(&self) -> Result<[u8; 3], Error> {
        Ok([ self.terrain, self.elevation, 0 ])
    }
}

impl Map {
    pub fn new(size: u32) -> Map {
        Map {
            size: size,
            tiles: Vec::with_capacity((size * size) as usize)
        }
    }

    pub fn tile_at(&self, x: u32, y: u32) -> Option<&MapTile> {
        let idx = (self.size * y + x) as usize;
        if idx < self.tiles.len() {
            Some(&self.tiles[idx])
        } else {
            None
        }
    }

    pub fn to_bytes(&self) -> Result<Vec<u8>, Error> {
        let size = self.size as usize;
        let mut buf = Vec::with_capacity(
            size * size * mem::size_of::<MapTile>() +
            2 * mem::size_of::<u32>()
        );
        try!(buf.write_u32::<LE>(self.size));
        try!(buf.write_u32::<LE>(self.size));
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
