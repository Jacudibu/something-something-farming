use crate::game::tilemap::tile_pos_to_world_pos;
use crate::prelude::chunk_identifier::ChunkIdentifier;
use crate::prelude::tilemap_layer::GroundLayer;
use crate::prelude::SpriteAssets;
use crate::prelude::{ChunkPos, CursorPos, MapPos, CHUNK_SIZE};
use crate::GameState;
use bevy::app::{App, First, Plugin};
use bevy::core::Name;
use bevy::math::{IVec2, Vec2, Vec4};
use bevy::prelude::{
    default, in_state, Color, Commands, Component, IntoSystemConfigs, OnEnter, Query, Res, Sprite,
    SpriteBundle, Transform, Vec4Swizzles, Visibility, With, Without,
};
use bevy_ecs_tilemap::map::{TilemapGridSize, TilemapSize, TilemapType};
use bevy_ecs_tilemap::prelude::{TilePos, TileStorage};

pub struct TileCursorPlugin;
impl Plugin for TileCursorPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Playing), initialize_cursor)
            .add_systems(
                First,
                update_tile_cursor
                    .after(crate::game::update_cursor_pos)
                    .run_if(in_state(GameState::Playing)),
            );
    }
}

#[derive(Component, Debug)]
pub struct TileCursor {
    pub pos: MapPos,
}

impl TileCursor {
    pub fn global_position(&self) -> IVec2 {
        IVec2::new(
            self.pos.chunk.x * CHUNK_SIZE as i32 + self.pos.tile.x as i32,
            self.pos.chunk.y * CHUNK_SIZE as i32 + self.pos.tile.y as i32,
        )
    }
}

fn initialize_cursor(mut commands: Commands, assets: Res<SpriteAssets>) {
    // TODO: Initialize Cursors only when tiles are actually selected
    commands.spawn((
        Name::new("Tile Cursor"),
        SpriteBundle {
            texture: assets.cursor.clone(),
            visibility: Visibility::Hidden,
            sprite: Sprite {
                color: Color::rgba(1.0, 1.0, 1.0, 0.25),
                ..default()
            },
            ..default()
        },
        TileCursor {
            pos: MapPos::new(ChunkPos::new(0, 0), TilePos::new(0, 0)),
        },
    ));
}

fn update_tile_cursor(
    cursor_pos: Res<CursorPos>,
    tilemap_q: Query<
        (
            &TilemapSize,
            &TilemapGridSize,
            &TilemapType,
            &TileStorage,
            &Transform,
            &ChunkIdentifier,
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
    for (map_size, grid_size, map_type, tile_storage, map_transform, chunk_identifier) in
        tilemap_q.iter()
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
                        tile_pos_to_world_pos(&tile_pos, &chunk_identifier.position, 100.0);
                    *visibility = Visibility::Visible;
                    cursor.pos.tile = tile_pos.clone();
                    cursor.pos.chunk = chunk_identifier.position.clone();
                }
            }
        }
    }
}
