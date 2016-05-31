use consts::{Terrain, MapSize};
use map::{MapTile, Map};

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
        for _ in 0..map.size {
            for _ in 0..map.size {
                map.tiles.push(MapTile::new(
                    self.base_terrain as u8,
                    self.base_elevation
                ));
            }
        }
        map
    }
}
