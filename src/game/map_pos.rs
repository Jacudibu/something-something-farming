use crate::game::{CHUNK_SIZE, TILE_SIZE};
use crate::prelude::ChunkPos;
use bevy::math::Vec3;
use bevy_ecs_tilemap::prelude::TilePos;

#[derive(Debug, Copy, Clone)]
pub struct MapPos {
    pub chunk: ChunkPos,
    pub tile: TilePos,
}

impl MapPos {
    pub fn new(chunk_pos: ChunkPos, tile_pos: TilePos) -> Self {
        MapPos {
            tile: tile_pos,
            chunk: chunk_pos,
        }
    }

    pub fn world_pos(&self, z: f32) -> Vec3 {
        Vec3 {
            x: self.chunk.x as f32 * CHUNK_SIZE as f32 * TILE_SIZE + self.tile.x as f32 * TILE_SIZE,
            y: self.chunk.y as f32 * CHUNK_SIZE as f32 * TILE_SIZE + self.tile.y as f32 * TILE_SIZE,
            z,
        }
    }
}
