use crate::prelude::helpers::determine_texture_index;
use crate::prelude::loaded_chunks::LoadedChunks;
use crate::prelude::tile_cursor::TileCursor;
use crate::prelude::tilemap_layer::GroundLayer;
use crate::prelude::update_tile_event::UpdateTileEvent;
use crate::prelude::{ActiveTool, ChunkPos, PlayerAction, WorldData};
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
            .add_event::<TileInteractionEvent>()
            .add_systems(Update, select_active_tool)
            .add_systems(Update, detect_tile_interactions)
            .add_systems(
                Update,
                process_tile_interactions.after(detect_tile_interactions),
            );
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

#[derive(Event, Debug)]
struct TileInteractionEvent {
    pub chunk_pos: ChunkPos,
    pub tile_pos: TilePos,
    pub used_tool: ActiveTool,
}

fn detect_tile_interactions(
    active_tool: Res<ActiveTool>,
    action_state: Query<&ActionState<PlayerAction>>,
    tile_cursor: Query<(&TileCursor, &Visibility)>,
    mut previously_interacted_tile: Local<Option<TilePos>>,
    mut tile_interaction_events: EventWriter<TileInteractionEvent>,
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

        // in case we ever regularly happening AoE interaction events, it batch_send might be more performant
        tile_interaction_events.send(TileInteractionEvent {
            tile_pos: cursor.tile_pos,
            chunk_pos: cursor.chunk_pos,
            used_tool: active_tool.deref().clone(),
        });
    }
}

fn process_tile_interactions(
    mut tile_interaction_event: EventReader<TileInteractionEvent>,
    mut commands: Commands,
    mut update_tile_events: EventWriter<UpdateTileEvent>,
    mut world_data: ResMut<WorldData>,
    mut object_chunks: Query<&mut TileStorage, Without<GroundLayer>>,
    loaded_chunk_data: Res<LoadedChunks>,
) {
    for event in tile_interaction_event.read() {
        match event.used_tool {
            ActiveTool::Hoe => {
                let world_data = &mut *world_data;
                let chunk = world_data.chunks.get_mut(&event.chunk_pos).unwrap();
                if chunk.at_pos(&event.tile_pos).is_tilled {
                    continue;
                }

                chunk.set_at_pos(&event.tile_pos, true);

                // -- update tiles --
                // This could happen in an event

                if let Some(loaded_data) = loaded_chunk_data.chunks.get(&event.chunk_pos) {
                    let mut floor_tilemap_storage =
                        object_chunks.get_mut(loaded_data.floor_tilemap).unwrap();

                    let tilled_tile = commands
                        .spawn(TileBundle {
                            position: event.tile_pos.clone(),
                            tilemap_id: TilemapId(loaded_data.floor_tilemap),
                            texture_index: determine_texture_index(
                                &event.tile_pos,
                                &event.chunk_pos,
                                &world_data,
                            ),
                            ..Default::default()
                        })
                        .id();
                    commands
                        .entity(loaded_data.floor_tilemap)
                        .add_child(tilled_tile);
                    floor_tilemap_storage.set(&event.tile_pos, tilled_tile);
                    update_tile_events.send_batch(UpdateTileEvent::surrounding_tiles(
                        event.chunk_pos,
                        event.tile_pos,
                    ));
                }
            }
            ActiveTool::Pickaxe => {
                let chunk = world_data.chunks.get_mut(&event.chunk_pos).unwrap();
                if !chunk.at_pos(&event.tile_pos).is_tilled {
                    continue;
                }

                chunk.set_at_pos(&event.tile_pos, false);

                if let Some(loaded_data) = loaded_chunk_data.chunks.get(&event.chunk_pos) {
                    let mut floor_tilemap_storage =
                        object_chunks.get_mut(loaded_data.floor_tilemap).unwrap();

                    if let Some(entity) = floor_tilemap_storage.get(&event.tile_pos) {
                        floor_tilemap_storage.remove(&event.tile_pos);
                        commands.entity(entity).despawn();
                    } else {
                        warn!("Entity was not set at {:?}.", event);
                    }

                    update_tile_events.send_batch(UpdateTileEvent::surrounding_tiles(
                        event.chunk_pos,
                        event.tile_pos,
                    ));
                }
            }
            ActiveTool::Seed => {
                let chunk = world_data.chunks.get_mut(&event.chunk_pos).unwrap();
                if !chunk.at_pos(&event.tile_pos).is_tilled {
                    continue;
                }

                info!("Planty planty!");
            }
        }
    }
}
