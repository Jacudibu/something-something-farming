use crate::game::tilemap::ground_type::GroundType;
use bevy::app::{App, Plugin};
use bevy::prelude::{IVec2, Resource};
use bevy::utils::{default, HashMap};

pub struct WorldDataPlugin;
impl Plugin for WorldDataPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<WorldData>();
    }
}

type ChunkPosition = IVec2;

#[derive(Resource)]
pub struct WorldData {
    pub chunks: HashMap<ChunkPosition, Chunk>,
}

impl Default for WorldData {
    fn default() -> Self {
        let mut result = WorldData { chunks: default() };

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

pub const CHUNK_SIZE: usize = 32;

pub struct Chunk {
    pub ground: [GroundType; CHUNK_SIZE * CHUNK_SIZE],
}

impl Default for Chunk {
    fn default() -> Self {
        Chunk {
            ground: [GroundType::Grass; CHUNK_SIZE * CHUNK_SIZE],
        }
    }
}
