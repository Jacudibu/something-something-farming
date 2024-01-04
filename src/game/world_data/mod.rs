use crate::prelude::chunk_data::Chunk;
use bevy::prelude::{App, IVec2, Plugin, Resource};
use bevy::utils::HashMap;

pub mod chunk_data;
pub mod ground_type;
pub mod tile_data;

pub type ChunkPosition = IVec2;

pub struct WorldDataPlugin;
impl Plugin for WorldDataPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<WorldData>();
    }
}

#[derive(Resource)]
pub struct WorldData {
    pub chunks: HashMap<ChunkPosition, Chunk>,
}

impl Default for WorldData {
    fn default() -> Self {
        let mut result = WorldData {
            chunks: HashMap::default(),
        };

        for x in [-1, 0] {
            for y in [-1, 0] {
                result
                    .chunks
                    .insert(ChunkPosition::new(x, y), Chunk::default());
            }
        }

        result
    }
}
