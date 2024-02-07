use bevy::math::Vec3;

use crate::game::CHUNK_SIZE;
use crate::prelude::{ChunkPos, TilePos};

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
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

    pub fn world_pos(&self, y: f32) -> Vec3 {
        Vec3 {
            x: self.chunk.x as f32 * CHUNK_SIZE as f32 + self.tile.x as f32,
            y,
            z: self.chunk.y as f32 * CHUNK_SIZE as f32 + self.tile.y as f32,
        }
    }

    pub fn pos_inside_chunk(&self, y: f32) -> Vec3 {
        Vec3 {
            x: self.tile.x as f32,
            y,
            z: self.tile.y as f32,
        }
    }
}
