use crate::game::tilemap::tile_pos_to_world_pos;
use crate::game::tilemap::tile_type::ChunkData;
use crate::game::CursorPos;
use bevy::app::{App, Plugin, Startup, Update};
use bevy::asset::{AssetServer, Handle};
use bevy::math::{Vec2, Vec4};
use bevy::prelude::{
    default, Color, Commands, Component, Entity, Image, Query, Res, Sprite, SpriteBundle,
    Transform, Vec4Swizzles, Visibility, With, Without,
};
use bevy_ecs_tilemap::map::{TilemapGridSize, TilemapSize, TilemapType};
use bevy_ecs_tilemap::prelude::{TilePos, TileStorage};

pub struct TileHighlightingPlugin;
impl Plugin for TileHighlightingPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, initialize_cursor)
            .add_systems(Update, highlight_tile_below_cursor);
    }
}

#[derive(Component)]
struct TileCursor {}

#[derive(Component)]
pub struct HighlightedTile;

fn initialize_cursor(mut commands: Commands, asset_server: Res<AssetServer>) {
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
