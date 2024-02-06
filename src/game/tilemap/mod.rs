use crate::game::tilemap::loaded_chunks::{LoadedChunkPlugin, LoadedChunks};
use crate::game::tilemap::update_tile_event::UpdateTileEventPlugin;
use crate::prelude::chunk_data::ChunkData;
use crate::prelude::chunk_identifier::ChunkIdentifier;
use crate::prelude::loaded_chunks::LoadedChunkData;
use crate::prelude::tile_cursor::TileCursorPlugin;
use crate::prelude::tilemap_layer::{GroundLayer, TilemapLayer};
use crate::prelude::{
    ChunkPos, DebugMaterials, SpriteAssets, WorldData, CHUNK_SIZE, DEBUG_WORLD_SIZE_MIN_AND_MAX,
    TILE_SIZE,
};
use crate::GameState;
use bevy::pbr::NotShadowCaster;
use bevy::prelude::*;
use bevy::utils::hashbrown::HashMap;
use bevy_ecs_tilemap::prelude::*;
use bevy_mod_raycast::deferred::DeferredRaycastingPlugin;
use bevy_mod_raycast::prelude::{RaycastMesh, RaycastPluginState};

pub(crate) mod chunk_identifier;
pub(crate) mod helpers;
pub(crate) mod loaded_chunks;
pub(crate) mod tile_cursor;
pub(crate) mod tilemap_layer;
pub(crate) mod update_tile_event;

const TILEMAP_TILE_SIZE: TilemapTileSize = TilemapTileSize {
    x: TILE_SIZE,
    y: TILE_SIZE,
};
const TILEMAP_SIZE: TilemapSize = TilemapSize {
    x: CHUNK_SIZE as u32,
    y: CHUNK_SIZE as u32,
};

const RENDER_CHUNK_SIZE: UVec2 = UVec2 {
    x: CHUNK_SIZE as u32 * 2,
    y: CHUNK_SIZE as u32 * 2,
};

#[derive(Reflect)]
pub struct TileRaycastSet;

pub struct GameMapPlugin;
impl Plugin for GameMapPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(TilemapRenderSettings {
            render_chunk_size: RENDER_CHUNK_SIZE,
            ..Default::default()
        })
        .add_plugins(DeferredRaycastingPlugin::<TileRaycastSet>::default())
        .add_plugins(TilemapPlugin)
        .add_plugins(TileCursorPlugin)
        .add_plugins(UpdateTileEventPlugin)
        .add_plugins(LoadedChunkPlugin)
        .add_systems(OnEnter(GameState::Playing), spawn_testing_chunks);
    }
}

fn tile_pos_to_world_pos(tile_pos: &TilePos, chunk_position: &ChunkPos, z: f32) -> Vec3 {
    Vec3::new(
        tile_pos.x as f32 * TILEMAP_TILE_SIZE.x
            + chunk_position.x as f32 * CHUNK_SIZE as f32 * TILEMAP_TILE_SIZE.x,
        tile_pos.y as f32 * TILEMAP_TILE_SIZE.y
            + chunk_position.y as f32 * CHUNK_SIZE as f32 * TILEMAP_TILE_SIZE.y,
        z,
    )
}

fn spawn_testing_chunks(
    mut commands: Commands,
    assets: Res<SpriteAssets>,
    world_data: Res<WorldData>,
    mut loaded_chunks: ResMut<LoadedChunks>,
    mut meshes: ResMut<Assets<Mesh>>,
    materials: Res<DebugMaterials>,
) {
    // FIXME: These should be created in the loading process
    // FIXME: Materials should use the texture from the spritesheet.
    let tile_mesh = meshes.add(shape::Plane::from_size(1.0).into());
    let tile_material = materials.single_tile.clone();

    for x in -DEBUG_WORLD_SIZE_MIN_AND_MAX..DEBUG_WORLD_SIZE_MIN_AND_MAX {
        for y in -DEBUG_WORLD_SIZE_MIN_AND_MAX..DEBUG_WORLD_SIZE_MIN_AND_MAX {
            spawn_chunk(
                &mut commands,
                &assets,
                ChunkPos::new(x, y),
                &world_data,
                &mut loaded_chunks,
                &tile_mesh,
                &tile_material,
            );
        }
    }
}

fn old_get_chunk_name(chunk_pos: ChunkPos, layer: TilemapLayer) -> Name {
    Name::new(format!("{} | {}", chunk_pos, layer))
}

fn get_chunk_name(chunk_pos: ChunkPos) -> Name {
    Name::new(format!("Chunk {}", chunk_pos))
}

fn despawn_chunk(mut commands: Commands, loaded_chunks: &mut LoadedChunks, chunk_pos: ChunkPos) {
    if let Some(chunk) = loaded_chunks.chunks.remove(&chunk_pos) {
        commands.entity(chunk.floor_tilemap).despawn_recursive();
        commands.entity(chunk.ground_tilemap).despawn_recursive();
    }
}

#[derive(Component, Debug)]
pub struct TilePos3D {
    x: u32,
    y: u32,
}

fn spawn_chunk(
    commands: &mut Commands,
    assets: &SpriteAssets,
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
                    TilePos3D {
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

    // Old Stuff vvv
    let ground_tilemap = spawn_ground_layer(commands, assets, chunk_pos, chunk_data);

    let floor_tilemap = commands
        .spawn((
            old_get_chunk_name(chunk_pos, TilemapLayer::Floor),
            TilemapLayer::Floor,
            ChunkIdentifier {
                position: chunk_pos,
            },
            TilemapBundle {
                grid_size: TILEMAP_TILE_SIZE.into(),
                size: TILEMAP_SIZE,
                texture: TilemapTexture::Single(assets.tilled_tiles.clone()),
                tile_size: TILEMAP_TILE_SIZE,
                transform: get_tilemap_transform(chunk_pos, TilemapLayer::Floor),
                storage: TileStorage::empty(TILEMAP_SIZE),
                ..Default::default()
            },
        ))
        .id();

    let loaded_chunk_data = LoadedChunkData {
        ground_tilemap,
        floor_tilemap,
        chunk_parent,
        tiles,
        crops: HashMap::new(),
    };

    loaded_chunks.chunks.insert(chunk_pos, loaded_chunk_data);
}

fn spawn_ground_layer(
    commands: &mut Commands,
    assets: &SpriteAssets,
    chunk_pos: ChunkPos,
    chunk: &ChunkData,
) -> Entity {
    let tilemap_entity = commands
        .spawn((
            old_get_chunk_name(chunk_pos, TilemapLayer::Ground),
            TilemapLayer::Ground,
            ChunkIdentifier {
                position: chunk_pos,
            },
            GroundLayer {},
        ))
        .id();
    let mut tile_storage = TileStorage::empty(TILEMAP_SIZE);

    for x in 0..CHUNK_SIZE as u32 {
        for y in 0..CHUNK_SIZE as u32 {
            let tile_pos = TilePos { x, y };
            let ground_type = &chunk.at(x, y).ground_type;
            let tile_entity = commands
                .spawn(TileBundle {
                    position: tile_pos,
                    tilemap_id: TilemapId(tilemap_entity),
                    texture_index: ground_type.texture_index(),
                    ..Default::default()
                })
                .id();
            commands.entity(tilemap_entity).add_child(tile_entity);
            tile_storage.set(&tile_pos, tile_entity);
        }
    }

    commands.entity(tilemap_entity).insert(TilemapBundle {
        grid_size: TILEMAP_TILE_SIZE.into(),
        size: TILEMAP_SIZE,
        storage: tile_storage,
        texture: TilemapTexture::Single(assets.simple_tiles.clone()),
        tile_size: TILEMAP_TILE_SIZE,
        transform: get_tilemap_transform(chunk_pos, TilemapLayer::Ground),
        ..Default::default()
    });

    tilemap_entity
}

fn get_tilemap_transform(chunk_pos: ChunkPos, layer: TilemapLayer) -> Transform {
    Transform::from_translation(Vec3::new(
        chunk_pos.x as f32 * CHUNK_SIZE as f32 * TILEMAP_TILE_SIZE.x,
        chunk_pos.y as f32 * CHUNK_SIZE as f32 * TILEMAP_TILE_SIZE.y,
        layer.into(),
    ))
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
