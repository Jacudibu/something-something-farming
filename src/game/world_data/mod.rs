use crate::prelude::chunk_data::ChunkData;
use bevy::prelude::{App, IVec2, Plugin, Resource};
use bevy::utils::HashMap;

pub mod chunk_data;
pub mod ground_type;
pub mod tile_data;

pub const DEBUG_WORLD_SIZE_MIN_AND_MAX: i32 = 1;

pub type ChunkPos = IVec2;

pub struct WorldDataPlugin;
impl Plugin for WorldDataPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<WorldData>();
    }
}

#[derive(Resource)]
pub struct WorldData {
    pub chunks: HashMap<ChunkPos, ChunkData>,
}

impl Default for WorldData {
    fn default() -> Self {
        let mut result = WorldData {
            chunks: HashMap::default(),
        };

        for x in -DEBUG_WORLD_SIZE_MIN_AND_MAX..DEBUG_WORLD_SIZE_MIN_AND_MAX {
            for y in -DEBUG_WORLD_SIZE_MIN_AND_MAX..DEBUG_WORLD_SIZE_MIN_AND_MAX {
                result
                    .chunks
                    .insert(ChunkPos::new(x, y), ChunkData::default());
            }
        }

        result
    }
}
