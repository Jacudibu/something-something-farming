use crate::game::tilemap::loaded_chunks::{LoadedChunkPlugin, LoadedChunks};
use crate::game::tilemap::update_tile_event::UpdateTileEventPlugin;
use crate::prelude::chunk_data::Chunk;
use crate::prelude::chunk_identifier::ChunkIdentifier;
use crate::prelude::loaded_chunks::LoadedChunkData;
use crate::prelude::tile_cursor::TileCursorPlugin;
use crate::prelude::tilemap_layer::{GroundLayer, TilemapLayer};
use crate::prelude::{ChunkPosition, WorldData, CHUNK_SIZE};
use bevy::prelude::*;
use bevy_ecs_tilemap::prelude::*;

pub(crate) mod chunk_identifier;
pub(crate) mod helpers;
pub(crate) mod loaded_chunks;
pub(crate) mod tile_cursor;
pub(crate) mod tilemap_layer;
pub(crate) mod update_tile_event;

const TILE_SIZE: TilemapTileSize = TilemapTileSize { x: 16.0, y: 16.0 };
const TILEMAP_SIZE: TilemapSize = TilemapSize {
    x: CHUNK_SIZE as u32,
    y: CHUNK_SIZE as u32,
};

const RENDER_CHUNK_SIZE: UVec2 = UVec2 {
    x: CHUNK_SIZE as u32 * 2,
    y: CHUNK_SIZE as u32 * 2,
};

pub struct GameMapPlugin;
impl Plugin for GameMapPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(TilemapRenderSettings {
            render_chunk_size: RENDER_CHUNK_SIZE,
            ..Default::default()
        })
        .add_plugins(TilemapPlugin)
        .add_plugins(TileCursorPlugin)
        .add_plugins(UpdateTileEventPlugin)
        .add_plugins(LoadedChunkPlugin)
        .add_systems(Startup, spawn_testing_chunks);
    }
}

fn tile_pos_to_world_pos(tile_pos: &TilePos, chunk_position: &IVec2, z: f32) -> Vec3 {
    Vec3::new(
        tile_pos.x as f32 * TILE_SIZE.x + chunk_position.x as f32 * CHUNK_SIZE as f32 * TILE_SIZE.x,
        tile_pos.y as f32 * TILE_SIZE.y + chunk_position.y as f32 * CHUNK_SIZE as f32 * TILE_SIZE.y,
        z,
    )
}

fn spawn_testing_chunks(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    world_data: Res<WorldData>,
    mut loaded_chunks: ResMut<LoadedChunks>,
) {
    spawn_chunk(
        &mut commands,
        &asset_server,
        ChunkPosition::new(0, 0),
        &world_data,
        &mut loaded_chunks,
    );
    spawn_chunk(
        &mut commands,
        &asset_server,
        ChunkPosition::new(0, -1),
        &world_data,
        &mut loaded_chunks,
    );
    spawn_chunk(
        &mut commands,
        &asset_server,
        ChunkPosition::new(-1, 0),
        &world_data,
        &mut loaded_chunks,
    );
    spawn_chunk(
        &mut commands,
        &asset_server,
        IVec2::new(-1, -1),
        &world_data,
        &mut loaded_chunks,
    );
}

fn get_chunk_name(chunk_pos: IVec2, layer: TilemapLayer) -> Name {
    Name::new(format!("{} | {}", chunk_pos, layer))
}

fn despawn_chunk(
    mut commands: Commands,
    loaded_chunks: &mut LoadedChunks,
    chunk_pos: ChunkPosition,
) {
    if let Some(chunk) = loaded_chunks.chunks.remove(&chunk_pos) {
        commands.entity(chunk.floor_tilemap).despawn_recursive();
        commands.entity(chunk.ground_tilemap).despawn_recursive();
    }
}

fn spawn_chunk(
    commands: &mut Commands,
    asset_server: &AssetServer,
    chunk_pos: ChunkPosition,
    world_data: &WorldData,
    loaded_chunks: &mut LoadedChunks,
) {
    let chunk_data = world_data
        .chunks
        .get(&chunk_pos)
        .expect(&format!("World data should exists for chunk {}", chunk_pos));

    let ground_tilemap = spawn_ground_layer(commands, asset_server, chunk_pos, chunk_data);

    let tilled_tile_texture: Handle<Image> = asset_server.load("sprites/tilled_tile.png");
    let floor_tilemap = commands
        .spawn((
            get_chunk_name(chunk_pos, TilemapLayer::Floor),
            TilemapLayer::Floor,
            ChunkIdentifier {
                position: chunk_pos,
            },
            TilemapBundle {
                grid_size: TILE_SIZE.into(),
                size: TILEMAP_SIZE,
                texture: TilemapTexture::Single(tilled_tile_texture),
                tile_size: TILE_SIZE,
                transform: get_tilemap_transform(chunk_pos, TilemapLayer::Floor),
                storage: TileStorage::empty(TILEMAP_SIZE),
                ..Default::default()
            },
        ))
        .id();

    let loaded_chunk_data = LoadedChunkData {
        ground_tilemap,
        floor_tilemap,
    };

    loaded_chunks.chunks.insert(chunk_pos, loaded_chunk_data);
}

fn spawn_ground_layer(
    commands: &mut Commands,
    asset_server: &AssetServer,
    chunk_pos: IVec2,
    chunk: &Chunk,
) -> Entity {
    let tilemap_entity = commands
        .spawn((
            get_chunk_name(chunk_pos, TilemapLayer::Ground),
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

    let tile_texture: Handle<Image> = asset_server.load("sprites/simple_tiles.png");
    commands.entity(tilemap_entity).insert(TilemapBundle {
        grid_size: TILE_SIZE.into(),
        size: TILEMAP_SIZE,
        storage: tile_storage,
        texture: TilemapTexture::Single(tile_texture),
        tile_size: TILE_SIZE,
        transform: get_tilemap_transform(chunk_pos, TilemapLayer::Ground),
        ..Default::default()
    });

    tilemap_entity
}

fn get_tilemap_transform(chunk_pos: IVec2, layer: TilemapLayer) -> Transform {
    Transform::from_translation(Vec3::new(
        chunk_pos.x as f32 * CHUNK_SIZE as f32 * TILE_SIZE.x,
        chunk_pos.y as f32 * CHUNK_SIZE as f32 * TILE_SIZE.y,
        layer.into(),
    ))
}
