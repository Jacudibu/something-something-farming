use crate::game::prelude::chunk_data::ChunkData;
use crate::game::prelude::tile_cursor::TileCursor;
use crate::game::prelude::tilemap_layer::{GroundLayer, TilemapLayer};
use bevy::prelude::*;
use bevy_ecs_tilemap::map::TilemapId;
use bevy_ecs_tilemap::prelude::TileBundle;
use bevy_ecs_tilemap::tiles::{TileStorage, TileTextureIndex};
use leafwing_input_manager::action_state::ActionState;
use leafwing_input_manager::axislike::{DeadZoneShape, DualAxis};
use leafwing_input_manager::input_map::InputMap;
use leafwing_input_manager::plugin::InputManagerPlugin;
use leafwing_input_manager::prelude::UserInput;
use leafwing_input_manager::user_input::InputKind;
use leafwing_input_manager::Actionlike;

#[derive(Actionlike, PartialEq, Eq, Hash, Clone, Copy, Debug, Reflect)]
pub enum PlayerAction {
    Move,
    Up,
    Down,
    Left,
    Right,
    Interact,
}

pub struct InteractionPlugin;
impl Plugin for InteractionPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(InputManagerPlugin::<PlayerAction>::default())
            .add_systems(Update, interact_with_tile);
    }
}

// TODO: Have a proper Chunk Entity which contains Entity References to all layers within the chunk, so we don't have to do this abomination here.
fn get_floor_layer_for_pos<'a>(
    query: &'a mut Query<
        (Entity, &ChunkData, &TilemapLayer, &mut TileStorage),
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
    query: Query<&ActionState<PlayerAction>>,
    tile_cursor: Query<(&TileCursor, &Visibility)>,
    mut object_chunks: Query<
        (Entity, &ChunkData, &TilemapLayer, &mut TileStorage),
        Without<GroundLayer>,
    >,
) {
    let action_state = query.get_single();
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

        let (tilled_tilemap, mut tilled_tilemap_storage) =
            get_floor_layer_for_pos(&mut object_chunks, cursor.chunk_pos).unwrap();

        if tilled_tilemap_storage.get(&cursor.tile_pos).is_some() {
            continue;
        }

        let tilled_tile = commands
            .spawn(TileBundle {
                position: cursor.tile_pos.clone(),
                tilemap_id: TilemapId(tilled_tilemap),
                texture_index: TileTextureIndex(0),
                ..Default::default()
            })
            .id();
        commands.entity(tilled_tilemap).add_child(tilled_tile);
        tilled_tilemap_storage.set(&cursor.tile_pos, tilled_tile);
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
