use std::fmt;
use std::fmt::Formatter;

use bevy::pbr::NotShadowCaster;
use bevy::prelude::*;
use bevy::utils::hashbrown::HashMap;
use bevy_mod_raycast::deferred::DeferredRaycastingPlugin;
use bevy_mod_raycast::prelude::RaycastMesh;

use crate::game::tilemap::loaded_chunks::{LoadedChunkPlugin, LoadedChunks};
use crate::game::tilemap::update_tile_event::UpdateTileEventPlugin;
use crate::prelude::chunk_identifier::ChunkIdentifier;
use crate::prelude::loaded_chunks::LoadedChunkData;
use crate::prelude::tile_cursor::TileCursorPlugin;
use crate::prelude::{
    ChunkPos, DebugMaterials, WorldData, CHUNK_SIZE, DEBUG_WORLD_SIZE_MIN_AND_MAX,
};
use crate::GameState;

pub(crate) mod chunk_identifier;
pub(crate) mod helpers;
pub(crate) mod loaded_chunks;
pub(crate) mod tile_cursor;
pub(crate) mod update_tile_event;

const RENDER_CHUNK_SIZE: UVec2 = UVec2 {
    x: CHUNK_SIZE as u32 * 2,
    y: CHUNK_SIZE as u32 * 2,
};

#[derive(Reflect)]
pub struct TileRaycastSet;

pub struct GameMapPlugin;
impl Plugin for GameMapPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(DeferredRaycastingPlugin::<TileRaycastSet>::default())
            .add_plugins(TileCursorPlugin)
            .add_plugins(UpdateTileEventPlugin)
            .add_plugins(LoadedChunkPlugin)
            .add_systems(OnEnter(GameState::Playing), spawn_testing_chunks);
    }
}

fn spawn_testing_chunks(
    mut commands: Commands,
    world_data: Res<WorldData>,
    mut loaded_chunks: ResMut<LoadedChunks>,
    mut meshes: ResMut<Assets<Mesh>>,
    materials: Res<DebugMaterials>,
) {
    // FIXME: These should be created in the loading process
    // FIXME: Materials should use the texture from the spritesheet.
    let tile_mesh = meshes.add(shape::Plane::from_size(1.0).into());
    let tile_material = materials.grass.clone();

    for x in -DEBUG_WORLD_SIZE_MIN_AND_MAX..DEBUG_WORLD_SIZE_MIN_AND_MAX {
        for y in -DEBUG_WORLD_SIZE_MIN_AND_MAX..DEBUG_WORLD_SIZE_MIN_AND_MAX {
            spawn_chunk(
                &mut commands,
                ChunkPos::new(x, y),
                &world_data,
                &mut loaded_chunks,
                &tile_mesh,
                &tile_material,
            );
        }
    }
}

fn get_chunk_name(chunk_pos: ChunkPos) -> Name {
    Name::new(format!("Chunk {}", chunk_pos))
}

fn despawn_chunk(mut commands: Commands, loaded_chunks: &mut LoadedChunks, chunk_pos: ChunkPos) {
    if let Some(chunk) = loaded_chunks.chunks.remove(&chunk_pos) {
        commands.entity(chunk.chunk_parent).despawn_recursive();
    }
}

#[derive(Component, Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct TilePos {
    pub x: u32,
    pub y: u32,
}

impl TilePos {
    pub fn new(x: u32, y: u32) -> Self {
        TilePos { x, y }
    }
}

impl fmt::Display for TilePos {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}|{}", self.x, self.y)
    }
}

fn spawn_chunk(
    commands: &mut Commands,
    chunk_pos: ChunkPos,
    world_data: &WorldData,
    loaded_chunks: &mut LoadedChunks,
    tile_mesh: &Handle<Mesh>,
    tile_material: &Handle<StandardMaterial>,
) {
    let chunk_data = world_data
        .chunks
        .get(&chunk_pos)
        .expect(&format!("World data should exists for chunk {}", chunk_pos));

    let mut tiles: [Option<Entity>; CHUNK_SIZE * CHUNK_SIZE] = [None; CHUNK_SIZE * CHUNK_SIZE];

    let chunk_parent = commands
        .spawn((
            get_chunk_name(chunk_pos),
            ChunkIdentifier {
                position: chunk_pos,
            },
            SpatialBundle {
                transform: get_chunk_transform(&chunk_pos),
                ..default()
            },
        ))
        .id();

    for x in 0..CHUNK_SIZE {
        for z in 0..CHUNK_SIZE {
            let entity = commands
                .spawn((
                    PbrBundle {
                        mesh: tile_mesh.clone(),
                        material: tile_material.clone(),
                        transform: get_tile_transform(x as f32, z as f32),
                        ..default()
                    },
                    TilePos {
                        x: x as u32,
                        y: z as u32,
                    },
                    NotShadowCaster,
                    RaycastMesh::<TileRaycastSet>::default(),
                ))
                .set_parent(chunk_parent)
                .id();

            tiles[x + z * CHUNK_SIZE] = Some(entity);
        }
    }

    let loaded_chunk_data = LoadedChunkData {
        chunk_parent,
        tiles,
        crops: HashMap::new(),
    };

    loaded_chunks.chunks.insert(chunk_pos, loaded_chunk_data);
}

fn get_chunk_transform(chunk_pos: &ChunkPos) -> Transform {
    Transform::from_xyz(
        chunk_pos.x as f32 * CHUNK_SIZE as f32,
        0.0,
        chunk_pos.y as f32 * CHUNK_SIZE as f32,
    )
}

fn get_tile_transform(x: f32, z: f32) -> Transform {
    Transform::from_xyz(x, 0.0, z)
}
