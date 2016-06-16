use std::iter::Iterator;

pub struct Coordinate {
    pub x: u32,
    pub y: u32
}

pub trait Selection {
    fn coordinates(&self) -> Vec<Coordinate>;
}

pub struct Rectangle {
    pub x: u32,
    pub y: u32,
    pub width: u32,
    pub height: u32,
}

impl Rectangle {
    pub fn new(x: u32, y: u32, width: u32, height: u32) -> Rectangle {
        Rectangle {
            x: x,
            y: y,
            width: width,
            height: height
        }
    }
}

impl Selection for Rectangle {
    fn coordinates(&self) -> Vec<Coordinate> {
        let mut tiles = vec![];
        for x in self.x..(self.x + self.width) {
            for y in self.y..(self.y + self.height) {
                tiles.push(Coordinate { x: x, y: y });
            }
        }
        tiles
    }
}
