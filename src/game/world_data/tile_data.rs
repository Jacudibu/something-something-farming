use crate::prelude::ground_type::GroundType;
use crate::prelude::CardinalDirection;

#[derive(Copy, Clone, Debug)]
pub struct TileData {
    pub ground_type: GroundType,
    pub is_tilled: bool,
    pub walls: TileWalls,
}

impl Default for TileData {
    fn default() -> Self {
        TileData {
            ground_type: GroundType::Grass,
            is_tilled: false,
            walls: TileWalls::default(),
        }
    }
}

#[derive(Debug, Default, Copy, Clone)]
pub struct TileWalls {
    pub north: bool,
    pub east: bool,
    pub south: bool,
    pub west: bool,
}

impl TileWalls {
    pub fn at(&self, direction: CardinalDirection) -> bool {
        match direction {
            CardinalDirection::North => self.north,
            CardinalDirection::East => self.east,
            CardinalDirection::South => self.south,
            CardinalDirection::West => self.west,
        }
    }

    pub fn set_at(&mut self, direction: CardinalDirection, value: bool) {
        match direction {
            CardinalDirection::North => self.north = value,
            CardinalDirection::East => self.east = value,
            CardinalDirection::South => self.south = value,
            CardinalDirection::West => self.west = value,
        }
    }
}
