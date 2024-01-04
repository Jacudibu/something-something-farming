use bevy_ecs_tilemap::prelude::TileTextureIndex;

#[derive(Debug, Copy, Clone)]
pub enum GroundType {
    Grass,
}

impl GroundType {
    pub fn texture_index(&self) -> TileTextureIndex {
        match self {
            GroundType::Grass => TileTextureIndex(2),
        }
    }
}
