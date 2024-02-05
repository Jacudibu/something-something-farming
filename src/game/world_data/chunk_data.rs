use crate::game::item_id::CropId;
use crate::prelude::tile_data::TileData;
use crate::prelude::CHUNK_SIZE;
use crate::prelude::{CropDefinition, SimulationTime};
use bevy::utils::HashMap;
use bevy_ecs_tilemap::prelude::TilePos;

pub struct ChunkData {
    pub tiles: [TileData; CHUNK_SIZE * CHUNK_SIZE],
    pub crops: HashMap<TilePos, CropData>,
}

pub struct CropData {
    pub crop_id: CropId,
    pub next_stage_at: Option<f32>,
    pub stage: u8,
}

impl CropData {
    pub fn new(from: &CropDefinition, simulation_time: &SimulationTime) -> Self {
        Self {
            crop_id: from.id.clone(),
            next_stage_at: Some(
                simulation_time.elapsed_seconds() + from.growth_time_per_stage as f32,
            ),
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
