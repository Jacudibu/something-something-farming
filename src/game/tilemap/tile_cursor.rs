use bevy::app::{App, First, Plugin};
use bevy::core::Name;
use bevy::math::{EulerRot, IVec2, Quat};
use bevy::pbr::NotShadowCaster;
use bevy::prelude::{
    default, error, in_state, Commands, Component, Entity, IntoSystemConfigs, Parent, Query, Res,
    Transform, With,
};
use bevy_mod_raycast::prelude::{RaycastMesh, RaycastSource};
use bevy_sprite3d::{Sprite3d, Sprite3dParams};

use crate::prelude::chunk_identifier::ChunkIdentifier;
use crate::prelude::{MapPos, TilePos, CHUNK_SIZE};
use crate::prelude::{SpriteAssets, TileRaycastSet};
use crate::GameState;

pub struct TileCursorPlugin;
impl Plugin for TileCursorPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
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

fn update_tile_cursor(
    mut commands: Commands,
    mut sprite_params: Sprite3dParams,
    assets: Res<SpriteAssets>,
    tile_ray: Query<&RaycastSource<TileRaycastSet>>,
    ray_targets: Query<(&TilePos, &Parent), With<RaycastMesh<TileRaycastSet>>>,
    chunk_parents: Query<&ChunkIdentifier>,
    tile_cursor_q: Query<(Entity, &TileCursor)>,
) {
    let mut this_frame_selection: Vec<MapPos> = Vec::new();

    for source in tile_ray.iter() {
        if let Some(intersections) = source.get_intersections() {
            for (entity, _) in intersections {
                match ray_targets.get(entity.clone()) {
                    Ok((tile_pos, parent)) => {
                        let chunk_identifier = chunk_parents.get(parent.get()).unwrap();
                        this_frame_selection
                            .push(MapPos::new(chunk_identifier.position, tile_pos.clone()));
                    }
                    Err(e) => {
                        error!("Unexpected error when raycasting for tile cursor: {}", e)
                    }
                }
            }
        }
    }

    let mut already_existing_cursors: Vec<MapPos> = Vec::new();
    for (entity, mut cursor) in tile_cursor_q.iter() {
        if this_frame_selection.contains(&cursor.pos) {
            already_existing_cursors.push(cursor.pos);
        } else {
            commands.entity(entity).despawn();
        }
    }

    for selected_tile in this_frame_selection.iter() {
        if already_existing_cursors.contains(selected_tile) {
            // Do nothing
        } else {
            commands.spawn((
                Name::new(format!(
                    "Tile Cursor | {} - {}",
                    selected_tile.chunk, selected_tile.tile
                )),
                Sprite3d {
                    image: assets.cursor.clone(),
                    unlit: true,
                    transform: Transform {
                        translation: selected_tile.world_pos(0.0),
                        rotation: Quat::from_euler(EulerRot::XYZ, 0.0, 0.0, 90.0),
                        ..default()
                    },
                    ..default()
                }
                .bundle(&mut sprite_params),
                TileCursor {
                    pos: selected_tile.clone(),
                },
                NotShadowCaster,
            ));
        }
    }
}
