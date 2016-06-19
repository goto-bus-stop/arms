use std::iter::Iterator;

#[derive(Clone, Copy, Debug)]
pub struct Coordinate {
    pub x: u32,
    pub y: u32
}

impl Coordinate {
    pub fn new(x: u32, y: u32) -> Coordinate {
        Coordinate { x: x, y: y }
    }
}

pub trait Selection : Sized + Copy {
    fn coordinates(&self) -> Vec<Coordinate>;

    fn and<B: Selection>(&self, b: B) -> AndSelection<Self, B> {
        AndSelection::new(*self, b)
    }
}

impl Selection for Coordinate {
    fn coordinates(&self) -> Vec<Coordinate> {
        vec![*self]
    }
}

#[derive(Clone, Copy, Debug)]
struct AndSelection<A: Selection, B: Selection> {
    a: A,
    b: B
}

impl<A: Selection, B: Selection> AndSelection<A, B> {
    fn new(a: A, b: B) -> AndSelection<A, B> {
        AndSelection { a: a, b: b }
    }
}

impl<A: Selection, B: Selection> Selection for AndSelection<A, B> {
    fn coordinates(&self) -> Vec<Coordinate> {
        let mut coords = vec![];
        for coord in self.a.coordinates() {
            coords.push(coord);
        }
        for coord in self.b.coordinates() {
            coords.push(coord);
        }
        coords
    }
}

#[derive(Clone, Copy, Debug)]
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
