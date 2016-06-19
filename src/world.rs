use consts::{Terrain, MapSize};
use map::{MapTile, Map};
use selection::{Rectangle, Selection};

pub struct World {
    pub base_terrain: Terrain,
    pub base_elevation: u8,
    pub map_size: u32
}

impl World {
    pub fn new() -> World {
        World {
            base_terrain: Terrain::Grass,
            base_elevation: 0,
            map_size: MapSize::Tiny as u32
        }
    }

    pub fn generate_map(&self) -> Map {
        let mut map = Map::new(self.map_size);
        for x in 0..map.size {
            for y in 0..map.size {
                map.tiles.push(MapTile::new(
                    if x == 20 {
                        Terrain::Snow as u8
                    } else if y == 20 {
                        Terrain::Dirt as u8
                    } else {
                        self.base_terrain as u8
                    },
                    self.base_elevation
                ));
            }
        }
        let selection = Rectangle::new(20, 20, 20, 20).and(Rectangle::new(5, 5, 5, 5));
        map.flatten_to(selection, 4);

        map.elevate(30, 30, 2);

        map
    }
}
