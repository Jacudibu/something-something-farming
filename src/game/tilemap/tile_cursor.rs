use crate::prelude::{ChunkPos, MapPos, TilePos3D, CHUNK_SIZE};
use crate::prelude::{SpriteAssets, TileRaycastSet};
use crate::GameState;
use bevy::app::{App, First, Plugin};
use bevy::core::Name;
use bevy::math::IVec2;
use bevy::prelude::{
    default, error, in_state, info, Color, Commands, Component, IntoSystemConfigs, OnEnter, Query,
    Res, Sprite, SpriteBundle, Transform, Visibility, With, Without,
};
use bevy_ecs_tilemap::map::TilemapSize;
use bevy_ecs_tilemap::prelude::TilePos;
use bevy_mod_raycast::prelude::{RaycastMesh, RaycastSource};

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
    tile_ray: Query<&RaycastSource<TileRaycastSet>>,
    ray_targets: Query<&TilePos3D, With<RaycastMesh<TileRaycastSet>>>,
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

    for source in tile_ray.iter() {
        if let Some(intersections) = source.get_intersections() {
            for (entity, data) in intersections {
                match ray_targets.get(entity.clone()) {
                    Ok(tile_pos) => {
                        info!("Hit! {:?}", tile_pos);
                        // FIXME: Actually visualize cursor in 3D
                        for (_transform, mut visibility, mut cursor) in tile_cursor_q.iter_mut() {
                            *visibility = Visibility::Visible;
                            cursor.pos.tile = TilePos::new(tile_pos.x, tile_pos.y);

                            // FIXME: Figure out chunk position. Maybe our TilePos3D is just a MapPos after all? Could also grab as a component from parent entity to avoid duplicate data.
                            cursor.pos.chunk = ChunkPos::new(0, 0);
                        }
                    }
                    Err(e) => {
                        error!("Unexpected error when raycasting for tile cursor: {}", e)
                    }
                }
                info!("Hit!");
            }
        }
    }
}
