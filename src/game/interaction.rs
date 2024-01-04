use crate::prelude::chunk_identifier::ChunkIdentifier;
use crate::prelude::tile_cursor::TileCursor;
use crate::prelude::tilemap_layer::{GroundLayer, TilemapLayer};
use crate::prelude::{ActiveTool, ChunkPosition, WorldData, CHUNK_SIZE};
use bevy::prelude::*;
use bevy_ecs_tilemap::map::TilemapId;
use bevy_ecs_tilemap::prelude::TileBundle;
use bevy_ecs_tilemap::tiles::{TilePos, TileStorage, TileTextureIndex};
use leafwing_input_manager::action_state::ActionState;
use leafwing_input_manager::axislike::{DeadZoneShape, DualAxis};
use leafwing_input_manager::input_map::InputMap;
use leafwing_input_manager::plugin::InputManagerPlugin;
use leafwing_input_manager::prelude::UserInput;
use leafwing_input_manager::user_input::InputKind;
use leafwing_input_manager::Actionlike;
use std::ops::Deref;

#[derive(Actionlike, PartialEq, Eq, Hash, Clone, Copy, Debug, Reflect)]
pub enum PlayerAction {
    Move,
    Up,
    Down,
    Left,
    Right,
    Interact,
    SelectHoe,
    SelectPickaxe,
}

pub struct InteractionPlugin;
impl Plugin for InteractionPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(InputManagerPlugin::<PlayerAction>::default())
            .insert_resource(ActiveTool::Hoe)
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
            }
        }
    }
}

// 00 01 02 03
// 04 05 06 07
// 08 09 10 11
// 12 13 14 15
fn determine_texture_index(
    pos: &TilePos,
    chunk_pos: &ChunkPosition,
    world_data: &WorldData,
) -> TileTextureIndex {
    let chunk = world_data.chunks.get(chunk_pos).unwrap();
    let up = if pos.y < CHUNK_SIZE as u32 - 1 {
        chunk.at(pos.x, pos.y + 1).is_tilled
    } else {
        let chunk = world_data
            .chunks
            .get(&ChunkPosition::new(chunk_pos.x, chunk_pos.y + 1));
        if let Some(chunk) = chunk {
            chunk.at(pos.x, 0).is_tilled
        } else {
            false
        }
    };
    let down = if pos.y > 0 {
        chunk.at(pos.x, pos.y - 1).is_tilled
    } else {
        let chunk = world_data
            .chunks
            .get(&ChunkPosition::new(chunk_pos.x, chunk_pos.y - 1));
        if let Some(chunk) = chunk {
            chunk.at(pos.x, CHUNK_SIZE as u32 - 1).is_tilled
        } else {
            false
        }
    };
    let right = if pos.x < CHUNK_SIZE as u32 - 1 {
        chunk.at(pos.x + 1, pos.y).is_tilled
    } else {
        let chunk = world_data
            .chunks
            .get(&ChunkPosition::new(chunk_pos.x + 1, chunk_pos.y));
        if let Some(chunk) = chunk {
            chunk.at(0, pos.y).is_tilled
        } else {
            false
        }
    };
    let left = if pos.x > 0 {
        chunk.at(pos.x - 1, pos.y).is_tilled
    } else {
        let chunk = world_data
            .chunks
            .get(&ChunkPosition::new(chunk_pos.x - 1, chunk_pos.y));
        if let Some(chunk) = chunk {
            chunk.at(CHUNK_SIZE as u32 - 1, pos.y).is_tilled
        } else {
            false
        }
    };

    if up {
        if down {
            if left {
                if right {
                    TileTextureIndex(10)
                } else {
                    TileTextureIndex(11)
                }
            } else if right {
                TileTextureIndex(9)
            } else {
                TileTextureIndex(8)
            }
        } else if left {
            if right {
                TileTextureIndex(14)
            } else {
                TileTextureIndex(15)
            }
        } else {
            if right {
                TileTextureIndex(13)
            } else {
                TileTextureIndex(12)
            }
        }
    } else if down {
        if left {
            if right {
                TileTextureIndex(6)
            } else {
                TileTextureIndex(7)
            }
        } else if right {
            TileTextureIndex(5)
        } else {
            TileTextureIndex(4)
        }
    } else if left {
        if right {
            TileTextureIndex(2)
        } else {
            TileTextureIndex(3)
        }
    } else {
        if right {
            TileTextureIndex(1)
        } else {
            TileTextureIndex(0)
        }
    }
}

pub fn default_input_map() -> InputMap<PlayerAction> {
    let mut input_map = InputMap::default();

    input_map.insert(
        UserInput::Single(InputKind::DualAxis(DualAxis::left_stick().with_deadzone(
            DeadZoneShape::Ellipse {
                radius_x: 0.1,
                radius_y: 0.1,
            },
        ))),
        PlayerAction::Move,
    );
    // input_map.insert(UserInput::VirtualDPad(VirtualDPad::wasd()), Action::Move);
    // input_map.insert(UserInput::VirtualDPad(VirtualDPad::arrow_keys()), Action::Move);
    // input_map.insert(UserInput::VirtualDPad(VirtualDPad::dpad()), Action::Move);

    input_map.insert(MouseButton::Left, PlayerAction::Interact);

    input_map.insert(KeyCode::Key1, PlayerAction::SelectHoe);
    input_map.insert(KeyCode::Key2, PlayerAction::SelectPickaxe);

    input_map.insert(KeyCode::Up, PlayerAction::Up);
    input_map.insert(KeyCode::W, PlayerAction::Up);
    input_map.insert(GamepadButtonType::DPadUp, PlayerAction::Up);

    input_map.insert(KeyCode::Down, PlayerAction::Down);
    input_map.insert(KeyCode::S, PlayerAction::Down);
    input_map.insert(GamepadButtonType::DPadDown, PlayerAction::Down);

    input_map.insert(KeyCode::Left, PlayerAction::Left);
    input_map.insert(KeyCode::A, PlayerAction::Left);
    input_map.insert(GamepadButtonType::DPadLeft, PlayerAction::Left);

    input_map.insert(KeyCode::Right, PlayerAction::Right);
    input_map.insert(KeyCode::D, PlayerAction::Right);
    input_map.insert(GamepadButtonType::DPadRight, PlayerAction::Right);

    input_map
}
