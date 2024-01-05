use crate::prelude::helpers::determine_texture_index;
use crate::prelude::loaded_chunks::LoadedChunks;
use crate::prelude::tile_cursor::TileCursor;
use crate::prelude::tilemap_layer::GroundLayer;
use crate::prelude::update_tile_event::UpdateTileEvent;
use crate::prelude::{ActiveTool, PlayerAction, WorldData};
use bevy::prelude::*;
use bevy_ecs_tilemap::map::TilemapId;
use bevy_ecs_tilemap::prelude::TileBundle;
use bevy_ecs_tilemap::tiles::{TilePos, TileStorage};
use leafwing_input_manager::action_state::ActionState;
use std::ops::Deref;

pub struct InteractionPlugin;
impl Plugin for InteractionPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(ActiveTool::Hoe)
            .add_systems(Update, select_active_tool)
            .add_systems(Update, interact_with_tile);
    }
}

fn select_active_tool(
    mut active_tool: ResMut<ActiveTool>,
    action_state: Query<&ActionState<PlayerAction>>,
) {
    let action_state = action_state.get_single();
    if action_state.is_err() {
        error!("PlayerAction State was missing!");
        return;
    }
    let action_state = action_state.unwrap();
    if action_state.just_pressed(PlayerAction::SelectHoe) {
        *active_tool = ActiveTool::Hoe;
    } else if action_state.just_pressed(PlayerAction::SelectPickaxe) {
        *active_tool = ActiveTool::Pickaxe;
    } else if action_state.just_pressed(PlayerAction::SelectSeed) {
        *active_tool = ActiveTool::Seed;
    }
}

fn interact_with_tile(
    mut commands: Commands,
    mut update_tile_events: EventWriter<UpdateTileEvent>,
    mut world_data: ResMut<WorldData>,
    active_tool: Res<ActiveTool>,
    action_state: Query<&ActionState<PlayerAction>>,
    tile_cursor: Query<(&TileCursor, &Visibility)>,
    mut object_chunks: Query<&mut TileStorage, Without<GroundLayer>>,
    loaded_chunk_data: Res<LoadedChunks>,
    mut previously_interacted_tile: Local<Option<TilePos>>,
) {
    let action_state = action_state.get_single();
    if action_state.is_err() {
        error!("PlayerAction State was missing!");
        return;
    }
    let action_state = action_state.unwrap();

    if !action_state.pressed(PlayerAction::Interact) {
        return;
    }

    if action_state.just_pressed(PlayerAction::Interact) {
        *previously_interacted_tile = None;
    }

    for (cursor, visibility) in tile_cursor.iter() {
        if visibility == Visibility::Hidden {
            continue;
        }

        if let Some(previous) = *previously_interacted_tile {
            if previous == cursor.tile_pos {
                return;
            } else {
                *previously_interacted_tile = Some(cursor.tile_pos);
            }
        } else {
            *previously_interacted_tile = Some(cursor.tile_pos);
        }

        // Could we technically just spawn a "TileInteractionEvent(ActiveTool, TileCursor) event right here?
        // Could this maybe even be generic on the active tool instance, so we have a "pickaxe listener" and a "water can listener"?

        match active_tool.deref() {
            ActiveTool::Hoe => {
                let world_data = &mut *world_data;
                let chunk = world_data.chunks.get_mut(&cursor.chunk_pos).unwrap();
                if chunk.at_pos(&cursor.tile_pos).is_tilled {
                    continue;
                }

                chunk.set_at_pos(&cursor.tile_pos, true);

                // -- update tiles --
                // This could happen in an event

                if let Some(loaded_data) = loaded_chunk_data.chunks.get(&cursor.chunk_pos) {
                    let mut floor_tilemap_storage =
                        object_chunks.get_mut(loaded_data.floor_tilemap).unwrap();

                    let tilled_tile = commands
                        .spawn(TileBundle {
                            position: cursor.tile_pos.clone(),
                            tilemap_id: TilemapId(loaded_data.floor_tilemap),
                            texture_index: determine_texture_index(
                                &cursor.tile_pos,
                                &cursor.chunk_pos,
                                &world_data,
                            ),
                            ..Default::default()
                        })
                        .id();
                    commands
                        .entity(loaded_data.floor_tilemap)
                        .add_child(tilled_tile);
                    floor_tilemap_storage.set(&cursor.tile_pos, tilled_tile);
                    update_tile_events.send_batch(UpdateTileEvent::surrounding_tiles(
                        cursor.chunk_pos,
                        cursor.tile_pos,
                    ));
                }
            }
            ActiveTool::Pickaxe => {
                let chunk = world_data.chunks.get_mut(&cursor.chunk_pos).unwrap();
                if !chunk.at_pos(&cursor.tile_pos).is_tilled {
                    continue;
                }

                chunk.set_at_pos(&cursor.tile_pos, false);

                if let Some(loaded_data) = loaded_chunk_data.chunks.get(&cursor.chunk_pos) {
                    let mut floor_tilemap_storage =
                        object_chunks.get_mut(loaded_data.floor_tilemap).unwrap();

                    if let Some(entity) = floor_tilemap_storage.get(&cursor.tile_pos) {
                        floor_tilemap_storage.remove(&cursor.tile_pos);
                        commands.entity(entity).despawn();
                    } else {
                        warn!("Entity was not set at {:?}.", cursor);
                    }

                    update_tile_events.send_batch(UpdateTileEvent::surrounding_tiles(
                        cursor.chunk_pos,
                        cursor.tile_pos,
                    ));
                }
            }
            ActiveTool::Seed => {
                let chunk = world_data.chunks.get_mut(&cursor.chunk_pos).unwrap();
                if !chunk.at_pos(&cursor.tile_pos).is_tilled {
                    continue;
                }

                info!("Planty planty!");
            }
        }
    }
}
