use crate::prelude::tile_data::TileData;
use crate::prelude::CHUNK_SIZE;
use bevy::prelude::Time;
use bevy::utils::HashMap;
use bevy_ecs_tilemap::prelude::TilePos;

pub struct ChunkData {
    pub tiles: [TileData; CHUNK_SIZE * CHUNK_SIZE],
    pub crops: HashMap<TilePos, CropData>,
}

pub struct CropData {
    pub next_stage_at: Option<f32>,
    pub stage: u8,
}

impl CropData {
    pub fn new(time: &Time) -> Self {
        Self {
            next_stage_at: Some(time.elapsed_seconds() + 5.0),
            stage: 0,
        }
    }
}

impl ChunkData {
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

impl Default for ChunkData {
    fn default() -> Self {
        ChunkData {
            tiles: [TileData::default(); CHUNK_SIZE * CHUNK_SIZE],
            crops: HashMap::new(),
        }
    }
}
