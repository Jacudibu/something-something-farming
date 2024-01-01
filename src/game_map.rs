use crate::game::CursorPos;
use bevy::prelude::*;
use bevy_ecs_tilemap::prelude::*;

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
        .add_systems(Update, highlight_tile_below_cursor)
        .add_systems(Startup, spawn_testing_chunks);
    }
}

#[derive(Component)]
pub struct HighlightedTile;

fn highlight_tile_below_cursor(
    mut commands: Commands,
    cursor_pos: Res<CursorPos>,
    tilemap_q: Query<(
        &TilemapSize,
        &TilemapGridSize,
        &TilemapType,
        &TileStorage,
        &Transform,
        &ChunkData,
    )>,
    highlighted_tiles_q: Query<Entity, (With<HighlightedTile>, Without<TileCursor>)>,
    mut tile_cursor_q: Query<
        (&mut Transform, &mut Visibility),
        (
            With<TileCursor>,
            Without<HighlightedTile>,
            Without<TilemapSize>,
        ),
    >,
) {
    // Un-highlight any previously highlighted tile labels.
    // TODO: Remove/Add highlights after detecting which tiles are selected this frame.
    for entity in highlighted_tiles_q.iter() {
        commands.entity(entity).remove::<HighlightedTile>();
    }

    for (_, mut visibility) in tile_cursor_q.iter_mut() {
        *visibility = Visibility::Hidden;
    }

    let cursor_pos: Vec4 = Vec4::from((cursor_pos.world, 0.0, 1.0));
    // TODO: Once we have multiple layers we might to only want to query the ground layer for this kind of selection.
    for (map_size, grid_size, map_type, tile_storage, map_transform, chunk_data) in tilemap_q.iter()
    {
        // We need to make sure that the cursor's world position is correct relative to the map
        // due to any map transformation.
        let cursor_pos_relative_to_tilemap: Vec2 = {
            let cursor_in_map_pos = map_transform.compute_matrix().inverse() * cursor_pos;
            cursor_in_map_pos.xy()
        };
        // Once we have a world position we can transform it into a possible tile position.
        if let Some(tile_pos) = TilePos::from_world_pos(
            &cursor_pos_relative_to_tilemap,
            map_size,
            grid_size,
            map_type,
        ) {
            // Highlight the relevant tile's label
            if let Some(tile_entity) = tile_storage.get(&tile_pos) {
                commands.entity(tile_entity).insert(HighlightedTile);

                for (mut transform, mut visibility) in tile_cursor_q.iter_mut() {
                    transform.translation =
                        tile_pos_to_world_pos(&tile_pos, &chunk_data.position, 100.0);
                    *visibility = Visibility::Visible;
                }
            }
        }
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

#[derive(Component)]
struct TileCursor {}

fn spawn_testing_chunks(mut commands: Commands, asset_server: Res<AssetServer>) {
    spawn_chunk(&mut commands, &asset_server, IVec2::new(0, 0));
    spawn_chunk(&mut commands, &asset_server, IVec2::new(0, -1));
    spawn_chunk(&mut commands, &asset_server, IVec2::new(-1, 0));
    spawn_chunk(&mut commands, &asset_server, IVec2::new(-1, -1));

    let tile_cursor_texture: Handle<Image> = asset_server.load("sprites/tile_cursor.png");
    commands
        .spawn(SpriteBundle {
            texture: tile_cursor_texture,
            visibility: Visibility::Hidden,
            sprite: Sprite {
                color: Color::rgba(1.0, 1.0, 1.0, 0.25),
                ..default()
            },
            ..default()
        })
        .insert(TileCursor {});
}

#[derive(Debug)]
pub enum TileType {
    Grass,
}

impl TileType {
    fn texture_index(&self) -> TileTextureIndex {
        match self {
            TileType::Grass => TileTextureIndex(2),
        }
    }
}

#[derive(Debug, Component)]
pub struct TileData {
    pub tile_type: TileType,
}

#[derive(Component)]
pub struct ChunkData {
    pub position: IVec2,
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
