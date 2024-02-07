use bevy::prelude::{App, Entity, Plugin, Resource};
use bevy::utils::HashMap;

use crate::game::CHUNK_SIZE;
use crate::prelude::{ChunkPos, TilePos};

pub struct LoadedChunkPlugin;
impl Plugin for LoadedChunkPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(LoadedChunks {
            chunks: HashMap::new(),
        });
    }
}

#[derive(Resource)]
pub struct LoadedChunks {
    pub chunks: HashMap<ChunkPos, LoadedChunkData>,
}

pub struct LoadedChunkData {
    pub chunk_parent: Entity,
    pub tiles: [Option<Entity>; CHUNK_SIZE * CHUNK_SIZE],
    pub crops: HashMap<TilePos, Entity>,
}

impl LoadedChunkData {
    pub fn get_tile(&self, x: u32, y: u32) -> Option<Entity> {
        debug_assert!(
            (x as usize) < CHUNK_SIZE && (y as usize) < CHUNK_SIZE,
            "Invalid pos: {}, {}",
            x,
            y
        );
        self.tiles[x as usize + y as usize * CHUNK_SIZE]
    }
}
