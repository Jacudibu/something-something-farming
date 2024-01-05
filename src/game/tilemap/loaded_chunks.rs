use crate::prelude::ChunkPos;
use bevy::prelude::{App, Entity, Plugin, Resource};
use bevy::utils::HashMap;

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
    pub ground_tilemap: Entity,
    pub floor_tilemap: Entity,
}
