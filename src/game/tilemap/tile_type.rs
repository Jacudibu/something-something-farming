use bevy::math::IVec2;
use bevy::prelude::Component;
use bevy_ecs_tilemap::prelude::TileTextureIndex;

#[derive(Debug)]
pub enum TileType {
    Grass,
}

impl TileType {
    pub fn texture_index(&self) -> TileTextureIndex {
        match self {
            TileType::Grass => TileTextureIndex(2),
        }
    }
}

#[derive(Debug, Component)]
pub struct TileData {
    pub tile_type: TileType,
}

#[derive(Component)]
pub struct ChunkData {
    pub position: IVec2,
}
