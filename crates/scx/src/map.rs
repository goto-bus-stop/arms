use std::io::{Write, Error};
use std::mem;
use byteorder::{LittleEndian as LE, WriteBytesExt};

use selection::Selection;

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

    pub fn with_terrain(&self, terrain: u8) -> MapTile {
        MapTile {
            terrain: terrain,
            elevation: self.elevation
        }
    }

    pub fn with_elevation(&self, elevation: u8) -> MapTile {
        MapTile {
            terrain: self.terrain,
            elevation: elevation
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

    pub fn tile_at(&self, x: u32, y: u32) -> Option<MapTile> {
        let idx = (self.size * y + x) as usize;
        if idx < self.tiles.len() {
            Some(MapTile::new(self.tiles[idx].terrain, self.tiles[idx].elevation))
        } else {
            None
        }
    }

    pub fn put_tile(&mut self, x: u32, y: u32, tile: MapTile) {
        let idx = (self.size * y + x) as usize;
        if idx < self.tiles.len() {
            self.tiles[idx] = tile;
        }
    }

    pub fn elevation_at(&self, x: u32, y: u32) -> Option<u8> {
        match self.tile_at(x, y) {
            Some(tile) => Some(tile.elevation),
            None => None
        }
    }


    pub fn tile_neighbours(&self, x: u32, y: u32) -> Vec<(u32, u32)> {
        let candidates = vec![
            (x - 1, y - 1),
            (x - 1, y),
            (x - 1, y + 1),
            (x, y - 1),
            (x, y + 1),
            (x + 1, y - 1),
            (x + 1, y),
            (x + 1, y + 1),
        ];
        let mut neighbours = vec![];
        for candidate in candidates {
            if candidate.0 < self.size && candidate.1 < self.size {
                neighbours.push(candidate)
            }
        }
        neighbours
    }

    pub fn elevate_raw(&mut self, x: u32, y: u32, elevation: u8) {
        match self.tile_at(x, y) {
            Some(tile) => {
                self.put_tile(x, y, tile.with_elevation(elevation));
            },
            None => ()
        }
    }

    pub fn elevate(&mut self, x: u32, y: u32, elevation: u8) {
        match self.tile_at(x, y) {
            Some(tile) => {
                self.put_tile(x, y, tile.with_elevation(elevation));
                for c in self.tile_neighbours(x, y) {
                    let neighbour = self.tile_at(c.0, c.1).unwrap();
                    let current_difference = elevation as i8 - neighbour.elevation as i8;
                    if current_difference.abs() > 1 {
                        let desired_difference: i8 = if neighbour.elevation > elevation { 1 } else { -1 };
                        self.elevate(c.0, c.1, (elevation as i8 + desired_difference) as u8);
                    }
                }
            },
            None => ()
        }
    }

    pub fn flatten<T: Selection>(&mut self, selection: T) {
        let mut weighted_elevation = 0;
        let mut tiles = 0;
        for coord in selection.coordinates() {
            match self.tile_at(coord.x, coord.y) {
                Some(tile) => {
                    tiles += 1;
                    weighted_elevation += tile.elevation as u32;
                },
                None => ()
            }
        }
        self.flatten_to(selection, (weighted_elevation / tiles) as u8)
    }
    pub fn flatten_to<T: Selection>(&mut self, selection: T, elevation: u8) {
        for coord in selection.coordinates() {
            self.elevate(coord.x, coord.y, elevation)
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
