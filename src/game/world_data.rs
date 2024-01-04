use crate::game::tilemap::ground_type::GroundType;
use bevy::app::{App, Plugin};
use bevy::prelude::{IVec2, Resource};
use bevy::utils::{default, HashMap};
use bevy_ecs_tilemap::tiles::TilePos;

pub struct WorldDataPlugin;
impl Plugin for WorldDataPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<WorldData>();
    }
}

pub type ChunkPosition = IVec2;

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
    pub is_tilled: [bool; CHUNK_SIZE * CHUNK_SIZE], // TODO: PoC Placeholder, replace with something useful
}

impl Chunk {
    pub fn at(&self, x: u32, y: u32) -> bool {
        self.is_tilled[x as usize + y as usize * CHUNK_SIZE]
    }
    pub fn at_pos(&self, pos: &TilePos) -> bool {
        self.at(pos.x, pos.y)
    }
    pub fn set_at(&mut self, x: u32, y: u32, value: bool) {
        self.is_tilled[x as usize + y as usize * CHUNK_SIZE] = value;
    }
    pub fn set_at_pos(&mut self, pos: &TilePos, value: bool) {
        self.set_at(pos.x, pos.y, value);
    }
}

impl Default for Chunk {
    fn default() -> Self {
        Chunk {
            ground: [GroundType::Grass; CHUNK_SIZE * CHUNK_SIZE],
            is_tilled: [false; CHUNK_SIZE * CHUNK_SIZE],
        }
    }
}
