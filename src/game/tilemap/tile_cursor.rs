use bevy::app::{App, First, Plugin};
use bevy::core::Name;
use bevy::math::{IVec2, Quat};
use bevy::pbr::NotShadowCaster;
use bevy::prelude::*;
use bevy_mod_raycast::prelude::{RaycastMesh, RaycastSource};
use bevy_sprite3d::{Sprite3d, Sprite3dParams};

use crate::prelude::chunk_identifier::ChunkIdentifier;
use crate::prelude::{
    CardinalDirection, MapPos, MouseCursorOverUiState, TilePos, CHUNK_SIZE, SPRITE_PIXELS_PER_METER,
};
use crate::prelude::{SpriteAssets, TileRaycastSet};
use crate::GameState;

pub struct TileCursorPlugin;
impl Plugin for TileCursorPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            First,
            update_tile_cursor
                .after(crate::game::update_cursor_pos)
                .run_if(in_state(GameState::Playing))
                .run_if(in_state(MouseCursorOverUiState::NotOverUI)),
        );
    }
}

#[derive(Component, Debug)]
pub struct TileCursor {
    pub pos: MapPos,
    pub tile_edge: CardinalDirection,
    pub mouse_pos: Vec3,
}

impl TileCursor {
    pub fn global_position(&self) -> IVec2 {
        IVec2::new(
            self.pos.chunk.x * CHUNK_SIZE as i32 + self.pos.tile.x as i32,
            self.pos.chunk.y * CHUNK_SIZE as i32 + self.pos.tile.y as i32,
        )
    }
}

fn intersection_to_tile_edge(intersection_pos: Vec3) -> CardinalDirection {
    /*
         Turn the quad into 4 triangles and figure out which outer edge is the closest
               ______
         0    | \N /|
       z |    |W X E|
         1    |/ S\ |
              ‾‾‾‾‾‾
           0 ------- 1
                x
    */

    const TILE_ORIGIN_OFFSET: f32 = 0.5; // Tile origin is at [0.5,0.5]

    let x = {
        let x = (intersection_pos.x + TILE_ORIGIN_OFFSET).fract();
        if x < 0.0 {
            x + 1.0
        } else {
            x
        }
    };
    let z = {
        let z = (intersection_pos.z + TILE_ORIGIN_OFFSET).fract();
        if z < 0.0 {
            z + 1.0
        } else {
            z
        }
    };

    return if x < 0.5 {
        if z < 0.5 {
            if x < z {
                CardinalDirection::West
            } else {
                CardinalDirection::North
            }
        } else {
            if x + z > 1.0 {
                CardinalDirection::South
            } else {
                CardinalDirection::West
            }
        }
    } else {
        if z < 0.5 {
            if x + z > 1.0 {
                CardinalDirection::East
            } else {
                CardinalDirection::North
            }
        } else {
            if x < z {
                CardinalDirection::South
            } else {
                CardinalDirection::East
            }
        }
    };
}

fn update_tile_cursor(
    mut commands: Commands,
    mut sprite_params: Sprite3dParams,
    assets: Res<SpriteAssets>,
    tile_ray: Query<&RaycastSource<TileRaycastSet>>,
    ray_targets: Query<(&TilePos, &Parent), With<RaycastMesh<TileRaycastSet>>>,
    chunk_parents: Query<&ChunkIdentifier>,
    mut tile_cursor_q: Query<(Entity, &mut TileCursor)>,
) {
    // TODO: Reconsider having only one "Main Cursor" (mouse) - All other cursors should be something else and spawned from that main cursor
    let mut this_frame_selection: Vec<(MapPos, CardinalDirection, Vec3)> = Vec::new();

    for source in tile_ray.iter() {
        if let Some(intersections) = source.get_intersections() {
            for (entity, intersection) in intersections {
                let direction = intersection_to_tile_edge(intersection.position());
                match ray_targets.get(entity.clone()) {
                    Ok((tile_pos, parent)) => {
                        let chunk_identifier = chunk_parents.get(parent.get()).unwrap();
                        this_frame_selection.push((
                            MapPos::new(chunk_identifier.position, tile_pos.clone()),
                            direction,
                            intersection.position(),
                        ));
                    }
                    Err(e) => {
                        error!("Unexpected error when raycasting for tile cursor: {}", e)
                    }
                }
            }
        }
    }

    if this_frame_selection.is_empty() {
        return;
    }

    let mut already_existing_cursors: Vec<MapPos> = Vec::new();
    for (entity, mut cursor) in tile_cursor_q.iter_mut() {
        if let Some((_, edge, mouse)) = this_frame_selection
            .iter()
            .find(|(pos, _, _)| pos == &cursor.pos)
        {
            already_existing_cursors.push(cursor.pos);
            cursor.mouse_pos = mouse.clone();
            cursor.tile_edge = edge.clone();
        } else {
            commands.entity(entity).despawn();
        }
    }

    for (selected_tile, tile_edge, mouse_intersection) in this_frame_selection.iter() {
        if already_existing_cursors.contains(selected_tile) {
            // Update edge
        } else {
            commands.spawn((
                Name::new(format!(
                    "Tile Cursor {} > {}",
                    selected_tile.chunk, selected_tile.tile
                )),
                Sprite3d {
                    image: assets.cursor.clone(),
                    unlit: true,
                    pixels_per_metre: SPRITE_PIXELS_PER_METER,
                    transform: Transform {
                        translation: selected_tile.world_pos(0.0),
                        rotation: Quat::from_rotation_x(f32::to_radians(90.0)),
                        ..default()
                    },
                    ..default()
                }
                .bundle(&mut sprite_params),
                TileCursor {
                    pos: selected_tile.clone(),
                    tile_edge: tile_edge.clone(),
                    mouse_pos: mouse_intersection.clone(),
                },
                NotShadowCaster,
            ));
        }
    }
}
