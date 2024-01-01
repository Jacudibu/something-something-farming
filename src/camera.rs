use crate::game::CursorPos;
use bevy::prelude::*;
use bevy::render::camera::ScalingMode;
use leafwing_input_manager::action_state::ActionState;
use leafwing_input_manager::axislike::{DeadZoneShape, DualAxis};
use leafwing_input_manager::buttonlike::MouseWheelDirection;
use leafwing_input_manager::input_map::InputMap;
use leafwing_input_manager::plugin::InputManagerPlugin;
use leafwing_input_manager::prelude::UserInput;
use leafwing_input_manager::user_input::InputKind;
use leafwing_input_manager::{Actionlike, InputManagerBundle};

const SPEED: f32 = 50.0;
const SUPERSPEED_MULTIPLIER: f32 = 3.0;

pub struct CameraPlugin;
impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(InputManagerPlugin::<CameraAction>::default())
            .add_systems(Startup, init)
            .add_systems(Update, zoom_camera)
            .add_systems(Last, move_camera);
    }
}

#[derive(Component)]
pub struct CameraFocus {}

#[derive(Actionlike, PartialEq, Eq, Hash, Clone, Copy, Debug, Reflect)]
pub enum CameraAction {
    ZoomIn,
    ZoomOut,
    Move,
    Superspeed,
    Up,
    Down,
    Left,
    Right,
}

fn init(mut commands: Commands) {
    let mut camera = Camera2dBundle::default();
    camera.projection.scaling_mode = ScalingMode::WindowSize(2.0);

    commands
        .spawn(camera)
        .insert(InputManagerBundle::<CameraAction> {
            input_map: default_input_map_camera(),
            ..default()
        });
}

fn move_camera(
    time: Res<Time>,
    mut query: Query<(&mut Transform, &ActionState<CameraAction>), With<Camera2d>>,
    mut cursor_pos: ResMut<CursorPos>,
) {
    let (mut transform, action_state) = query.single_mut();
    let mut dir;
    if action_state.pressed(CameraAction::Move) {
        dir = action_state
            .clamped_axis_pair(CameraAction::Move)
            .unwrap()
            .xy()
            .extend(0.0);
    } else {
        dir = Vec3::ZERO;
    }

    if action_state.pressed(CameraAction::Up) {
        dir.y += 1.0;
    }
    if action_state.pressed(CameraAction::Down) {
        dir.y -= 1.0;
    }
    if action_state.pressed(CameraAction::Right) {
        dir.x += 1.0;
    }
    if action_state.pressed(CameraAction::Left) {
        dir.x -= 1.0;
    }

    let speed = {
        if action_state.pressed(CameraAction::Superspeed) {
            SPEED * SUPERSPEED_MULTIPLIER
        } else {
            SPEED
        }
    };
    let delta = {
        if dir.length() > 1.0 {
            if let Some(dir) = dir.try_normalize() {
                dir * speed * time.delta_seconds()
            } else {
                Vec3::ZERO
            }
        } else {
            dir * speed * time.delta_seconds()
        }
    };

    transform.translation += delta;
    cursor_pos.world += delta.truncate();
}

const MAX_ZOOM: f32 = 4.0;
const MIN_ZOOM: f32 = 1.0;

fn zoom_camera(
    mut query: Query<(&mut OrthographicProjection, &ActionState<CameraAction>), With<Camera2d>>,
) {
    let (mut projection, action_state) = query.single_mut();

    let current_scaling = match projection.scaling_mode {
        ScalingMode::Fixed { .. } => 1.0,
        ScalingMode::WindowSize(x) => x,
        ScalingMode::AutoMin { .. } => 1.0,
        ScalingMode::AutoMax { .. } => 1.0,
        ScalingMode::FixedVertical(_) => 1.0,
        ScalingMode::FixedHorizontal(_) => 1.0,
    };

    if action_state.pressed(CameraAction::ZoomIn) && current_scaling < MAX_ZOOM {
        projection.scaling_mode = ScalingMode::WindowSize(current_scaling + 0.25)
    } else if action_state.pressed(CameraAction::ZoomOut) && current_scaling > MIN_ZOOM {
        projection.scaling_mode = ScalingMode::WindowSize(current_scaling - 0.25)
    }
}

fn default_input_map_camera() -> InputMap<CameraAction> {
    let mut input_map = InputMap::default();
    input_map.insert(MouseWheelDirection::Up, CameraAction::ZoomIn);
    input_map.insert(MouseWheelDirection::Down, CameraAction::ZoomOut);

    input_map.insert(
        UserInput::Single(InputKind::DualAxis(DualAxis::left_stick().with_deadzone(
            DeadZoneShape::Ellipse {
                radius_x: 0.1,
                radius_y: 0.1,
            },
        ))),
        CameraAction::Move,
    );
    // input_map.insert(UserInput::VirtualDPad(VirtualDPad::wasd()), Action::Move);
    // input_map.insert(UserInput::VirtualDPad(VirtualDPad::arrow_keys()), Action::Move);
    // input_map.insert(UserInput::VirtualDPad(VirtualDPad::dpad()), Action::Move);

    input_map.insert(KeyCode::ShiftLeft, CameraAction::Superspeed);

    input_map.insert(KeyCode::Up, CameraAction::Up);
    input_map.insert(KeyCode::W, CameraAction::Up);
    input_map.insert(GamepadButtonType::DPadUp, CameraAction::Up);

    input_map.insert(KeyCode::Down, CameraAction::Down);
    input_map.insert(KeyCode::S, CameraAction::Down);
    input_map.insert(GamepadButtonType::DPadDown, CameraAction::Down);

    input_map.insert(KeyCode::Left, CameraAction::Left);
    input_map.insert(KeyCode::A, CameraAction::Left);
    input_map.insert(GamepadButtonType::DPadLeft, CameraAction::Left);

    input_map.insert(KeyCode::Right, CameraAction::Right);
    input_map.insert(KeyCode::D, CameraAction::Right);
    input_map.insert(GamepadButtonType::DPadRight, CameraAction::Right);

    input_map
}
