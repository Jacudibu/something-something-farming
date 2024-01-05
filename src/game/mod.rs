use crate::game::camera::CameraPlugin;
use crate::game::interaction::InteractionPlugin;
use crate::game::tilemap::GameMapPlugin;
use crate::game::ui::UiPlugin;
use crate::game::world_data::WorldDataPlugin;
use bevy::app::{App, First, Plugin};
use bevy::math::Vec2;
use bevy::prelude::{
    Camera, CursorMoved, EventReader, GamepadButtonType, GlobalTransform, KeyCode, MouseButton,
    Query, Reflect, ResMut, Resource,
};
use leafwing_input_manager::axislike::{DeadZoneShape, DualAxis};
use leafwing_input_manager::input_map::InputMap;
use leafwing_input_manager::plugin::InputManagerPlugin;
use leafwing_input_manager::prelude::UserInput;
use leafwing_input_manager::user_input::InputKind;
use leafwing_input_manager::Actionlike;

pub mod active_tool;
pub mod camera;
pub mod interaction;
pub mod tilemap;
mod ui;
pub mod world_data;

pub const CHUNK_SIZE: usize = 32;

pub struct GamePlugin;
impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<CursorPos>()
            .add_plugins(InputManagerPlugin::<PlayerAction>::default())
            .add_plugins(WorldDataPlugin)
            .add_plugins(GameMapPlugin)
            .add_plugins(CameraPlugin)
            .add_plugins(InteractionPlugin)
            .add_plugins(UiPlugin)
            .add_systems(First, update_cursor_pos);
    }
}

#[derive(Resource)]
pub struct CursorPos {
    pub screen: Vec2,
    pub world: Vec2,
}
impl Default for CursorPos {
    fn default() -> Self {
        CursorPos {
            screen: Vec2::new(-10000.0, -10000.0),
            world: Vec2::new(-10000.0, -10000.0),
        }
    }
}

pub fn update_cursor_pos(
    camera_q: Query<(&GlobalTransform, &Camera)>,
    mut cursor_moved_events: EventReader<CursorMoved>,
    mut cursor_pos: ResMut<CursorPos>,
) {
    // TODO: If we are using the gamepad, cursorpos should be playerPos + stick * value
    for cursor_moved in cursor_moved_events.read() {
        cursor_pos.screen = cursor_moved.position;
        for (cam_t, cam) in camera_q.iter() {
            if let Some(pos) = cam.viewport_to_world_2d(cam_t, cursor_moved.position) {
                cursor_pos.world = pos;
            }
        }
    }
}

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
    SelectSeed,
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
    input_map.insert(KeyCode::Key3, PlayerAction::SelectSeed);

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
