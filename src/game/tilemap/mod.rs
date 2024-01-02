use crate::game::tilemap::tile_highlighting::TileHighlightingPlugin;
use bevy::prelude::*;
use bevy_ecs_tilemap::prelude::*;
use tile_type::{ChunkData, TileData, TileType};

pub mod tile_highlighting;
mod tile_type;

const TILE_SIZE: TilemapTileSize = TilemapTileSize { x: 16.0, y: 16.0 };
pub const CHUNK_SIZE: UVec2 = UVec2 { x: 32, y: 32 };
const RENDER_CHUNK_SIZE: UVec2 = UVec2 {
    x: CHUNK_SIZE.x * 2,
    y: CHUNK_SIZE.y * 2,
};

pub struct GameMapPlugin;
impl Plugin for GameMapPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(TilemapRenderSettings {
            render_chunk_size: RENDER_CHUNK_SIZE,
            ..Default::default()
        })
        .add_plugins(TilemapPlugin)
        .add_plugins(TileHighlightingPlugin)
        .add_systems(Startup, spawn_testing_chunks);
    }
}

fn tile_pos_to_world_pos(tile_pos: &TilePos, chunk_position: &IVec2, z: f32) -> Vec3 {
    Vec3::new(
        tile_pos.x as f32 * TILE_SIZE.x
            + chunk_position.x as f32 * CHUNK_SIZE.x as f32 * TILE_SIZE.x,
        tile_pos.y as f32 * TILE_SIZE.y
            + chunk_position.y as f32 * CHUNK_SIZE.y as f32 * TILE_SIZE.y,
        z,
    )
}

fn spawn_testing_chunks(mut commands: Commands, asset_server: Res<AssetServer>) {
    spawn_chunk(&mut commands, &asset_server, IVec2::new(0, 0));
    spawn_chunk(&mut commands, &asset_server, IVec2::new(0, -1));
    spawn_chunk(&mut commands, &asset_server, IVec2::new(-1, 0));
    spawn_chunk(&mut commands, &asset_server, IVec2::new(-1, -1));
}

fn spawn_chunk(commands: &mut Commands, asset_server: &AssetServer, chunk_pos: IVec2) {
    let tilemap_entity = commands.spawn_empty().id();
    let mut tile_storage = TileStorage::empty(CHUNK_SIZE.into());

    for x in 0..CHUNK_SIZE.x {
        for y in 0..CHUNK_SIZE.y {
            let tile_pos = TilePos { x, y };
            let tile_data = TileData {
                tile_type: TileType::Grass,
            };
            let tile_entity = commands
                .spawn(TileBundle {
                    position: tile_pos,
                    tilemap_id: TilemapId(tilemap_entity),
                    texture_index: tile_data.tile_type.texture_index(),
                    ..Default::default()
                })
                .insert(tile_data)
                .id();
            commands.entity(tilemap_entity).add_child(tile_entity);
            tile_storage.set(&tile_pos, tile_entity);
        }
    }

    let transform = Transform::from_translation(Vec3::new(
        chunk_pos.x as f32 * CHUNK_SIZE.x as f32 * TILE_SIZE.x,
        chunk_pos.y as f32 * CHUNK_SIZE.y as f32 * TILE_SIZE.y,
        0.0,
    ));

    let tile_texture: Handle<Image> = asset_server.load("sprites/simple_tiles.png");
    commands
        .entity(tilemap_entity)
        .insert(TilemapBundle {
            grid_size: TILE_SIZE.into(),
            size: CHUNK_SIZE.into(),
            storage: tile_storage,
            texture: TilemapTexture::Single(tile_texture),
            tile_size: TILE_SIZE,
            transform,
            ..Default::default()
        })
        .insert(ChunkData {
            position: chunk_pos,
        });
}
