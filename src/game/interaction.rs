use crate::prelude::chunk_identifier::ChunkIdentifier;
use crate::prelude::helpers::determine_texture_index;
use crate::prelude::tile_cursor::TileCursor;
use crate::prelude::tilemap_layer::{GroundLayer, TilemapLayer};
use crate::prelude::update_tile_event::UpdateTileEvent;
use crate::prelude::{ActiveTool, PlayerAction, WorldData};
use bevy::prelude::*;
use bevy_ecs_tilemap::map::TilemapId;
use bevy_ecs_tilemap::prelude::TileBundle;
use bevy_ecs_tilemap::tiles::TileStorage;
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
    }
}

// TODO: Have a proper Chunk Entity which contains Entity References to all layers within the chunk, so we don't have to do this abomination here.
fn get_floor_layer_for_pos<'a>(
    query: &'a mut Query<
        (Entity, &ChunkIdentifier, &TilemapLayer, &mut TileStorage),
        Without<GroundLayer>,
    >,
    target: IVec2,
) -> Option<(Entity, Mut<'a, TileStorage>)> {
    for (entity, data, layer, storage) in query.iter_mut() {
        if layer == &TilemapLayer::Floor && data.position == target {
            return Some((entity, storage));
        }
    }

    None
}

fn interact_with_tile(
    mut commands: Commands,
    mut world_data: ResMut<WorldData>,
    active_tool: Res<ActiveTool>,
    mut update_tile_events: EventWriter<UpdateTileEvent>,
    action_state: Query<&ActionState<PlayerAction>>,
    tile_cursor: Query<(&TileCursor, &Visibility)>,
    mut object_chunks: Query<
        (Entity, &ChunkIdentifier, &TilemapLayer, &mut TileStorage),
        Without<GroundLayer>,
    >,
) {
    let action_state = action_state.get_single();
    if action_state.is_err() {
        error!("PlayerAction State was missing!");
        return;
    }
    let action_state = action_state.unwrap();

    if !action_state.just_pressed(PlayerAction::Interact) {
        return;
    }

    for (cursor, visibility) in tile_cursor.iter() {
        if visibility == Visibility::Hidden {
            continue;
        }

        match active_tool.deref() {
            ActiveTool::Hoe => {
                let chunk = world_data.chunks.get_mut(&cursor.chunk_pos).unwrap();
                if chunk.at_pos(&cursor.tile_pos).is_tilled {
                    continue;
                }

                chunk.set_at_pos(&cursor.tile_pos, true);

                // -- update tiles --
                // This could happen in an event

                let (tilled_tilemap, mut tilled_tilemap_storage) =
                    get_floor_layer_for_pos(&mut object_chunks, cursor.chunk_pos).unwrap();

                let tilled_tile = commands
                    .spawn(TileBundle {
                        position: cursor.tile_pos.clone(),
                        tilemap_id: TilemapId(tilled_tilemap),
                        texture_index: determine_texture_index(
                            &cursor.tile_pos,
                            &cursor.chunk_pos,
                            &world_data,
                        ),
                        ..Default::default()
                    })
                    .id();
                commands.entity(tilled_tilemap).add_child(tilled_tile);
                tilled_tilemap_storage.set(&cursor.tile_pos, tilled_tile);
                update_tile_events.send_batch(UpdateTileEvent::surrounding_tiles(
                    cursor.chunk_pos,
                    cursor.tile_pos,
                ));
            }
            ActiveTool::Pickaxe => {
                let chunk = world_data.chunks.get_mut(&cursor.chunk_pos).unwrap();
                if !chunk.at_pos(&cursor.tile_pos).is_tilled {
                    continue;
                }

                chunk.set_at_pos(&cursor.tile_pos, false);

                let (_, mut tilled_tilemap_storage) =
                    get_floor_layer_for_pos(&mut object_chunks, cursor.chunk_pos).unwrap();

                if let Some(entity) = tilled_tilemap_storage.get(&cursor.tile_pos) {
                    tilled_tilemap_storage.remove(&cursor.tile_pos);
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
    }
}
