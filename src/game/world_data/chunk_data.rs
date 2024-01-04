use crate::prelude::tile_data::TileData;
use crate::prelude::CHUNK_SIZE;
use bevy_ecs_tilemap::prelude::TilePos;

pub struct Chunk {
    pub tiles: [TileData; CHUNK_SIZE * CHUNK_SIZE],
}

impl Chunk {
    pub fn at(&self, x: u32, y: u32) -> &TileData {
        &self.tiles[x as usize + y as usize * CHUNK_SIZE]
    }
    pub fn at_pos(&self, pos: &TilePos) -> &TileData {
        self.at(pos.x, pos.y)
    }
    pub fn set_at(&mut self, x: u32, y: u32, value: bool) {
        self.tiles[x as usize + y as usize * CHUNK_SIZE].is_tilled = value;
    }
    pub fn set_at_pos(&mut self, pos: &TilePos, value: bool) {
        self.set_at(pos.x, pos.y, value);
    }
}

impl Default for Chunk {
    fn default() -> Self {
        Chunk {
            tiles: [TileData::default(); CHUNK_SIZE * CHUNK_SIZE],
        }
    }
}
