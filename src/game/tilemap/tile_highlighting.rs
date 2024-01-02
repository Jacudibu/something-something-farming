use crate::game::prelude::chunk_data::ChunkData;
use crate::game::tilemap::tile_pos_to_world_pos;
use crate::game::CursorPos;
use bevy::app::{App, Plugin, Startup, Update};
use bevy::asset::{AssetServer, Handle};
use bevy::math::{IVec2, Vec2, Vec4};
use bevy::prelude::{
    default, Color, Commands, Component, Image, Query, Res, Sprite, SpriteBundle, Transform,
    Vec4Swizzles, Visibility, With, Without,
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
pub struct TileCursor {
    pub tile_pos: TilePos,
    pub chunk_pos: IVec2,
}

#[derive(Component)]
pub struct GroundLayer;

fn initialize_cursor(mut commands: Commands, asset_server: Res<AssetServer>) {
    // TODO: Initialize Cursors only when tiles are actually selected
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
        .insert(TileCursor {
            chunk_pos: IVec2::new(-100, -100),
            tile_pos: TilePos::new(10000, 10000),
        });
}

fn highlight_tile_below_cursor(
    cursor_pos: Res<CursorPos>,
    tilemap_q: Query<
        (
            &TilemapSize,
            &TilemapGridSize,
            &TilemapType,
            &TileStorage,
            &Transform,
            &ChunkData,
        ),
        With<GroundLayer>,
    >,
    mut tile_cursor_q: Query<
        (&mut Transform, &mut Visibility, &mut TileCursor),
        Without<TilemapSize>,
    >,
) {
    // Un-highlight any previously highlighted tile labels.
    // TODO: Remove/Add cursors after detecting which tiles are selected this frame.
    for (_, mut visibility, _) in tile_cursor_q.iter_mut() {
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
            if tile_storage.get(&tile_pos).is_some() {
                for (mut transform, mut visibility, mut cursor) in tile_cursor_q.iter_mut() {
                    transform.translation =
                        tile_pos_to_world_pos(&tile_pos, &chunk_data.position, 100.0);
                    *visibility = Visibility::Visible;
                    cursor.tile_pos = tile_pos.clone();
                    cursor.chunk_pos = chunk_data.position.clone();
                }
            }
        }
    }
}
