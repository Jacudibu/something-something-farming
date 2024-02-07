use bevy::ecs::query::QuerySingleError;
use bevy::pbr::CascadeShadowConfigBuilder;
use bevy::prelude::*;
use bevy::render::camera::ScalingMode;
use bevy_basic_camera::CameraControllerPlugin;
use bevy_mod_raycast::prelude::RaycastSource;
use leafwing_input_manager::action_state::ActionState;
use leafwing_input_manager::axislike::{DeadZoneShape, DualAxis};
use leafwing_input_manager::buttonlike::MouseWheelDirection;
use leafwing_input_manager::input_map::InputMap;
use leafwing_input_manager::plugin::InputManagerPlugin;
use leafwing_input_manager::prelude::UserInput;
use leafwing_input_manager::user_input::InputKind;
use leafwing_input_manager::{Actionlike, InputManagerBundle};

use crate::game::CursorPos;
use crate::prelude::{GameState, MouseCursorOverUiState, TileRaycastSet};

const SPEED: f32 = 50.0;
const SUPERSPEED_MULTIPLIER: f32 = 3.0;

pub struct CameraPlugin;
impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(InputManagerPlugin::<CameraAction>::default())
            .add_plugins(CameraControllerPlugin)
            .add_systems(Startup, init)
            .add_systems(
                Update,
                zoom_camera
                    .run_if(in_state(MouseCursorOverUiState::NotOverUI))
                    .run_if(in_state(GameState::Playing)),
            )
            .add_systems(Last, move_camera.run_if(in_state(GameState::Playing)));
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

const CAMERA_OFFSET_TO_PLAYER: Vec3 = Vec3::new(0.0, 16.0, 20.0);

fn init(mut commands: Commands) {
    // FIXME: Once we figure things out, Global Lights should be spawned in their own plugin

    // Simple minded ez light rotations
    //  X    Y
    // -1.0  1 Morning
    // -1.2  0 Noon
    // -1.0 -1 Evening
    commands.spawn((
        Name::new("Directional Light"),
        DirectionalLightBundle {
            directional_light: DirectionalLight {
                shadows_enabled: true,
                ..default()
            },
            // TODO: Figure out some good looking values
            cascade_shadow_config: CascadeShadowConfigBuilder {
                first_cascade_far_bound: 20.0,
                minimum_distance: 1.0,
                ..default()
            }
            .build(),
            transform: Transform::from_rotation(Quat::from_euler(EulerRot::XYZ, 5.7, 0.3, 0.0)),
            ..default()
        },
    ));

    commands.spawn((
        Name::new("Camera"),
        Camera3dBundle {
            transform: Transform {
                translation: CAMERA_OFFSET_TO_PLAYER,
                rotation: Quat::from_rotation_x(-0.65),
                ..default()
            },
            projection: Projection::Orthographic(OrthographicProjection {
                scale: 1.0,
                scaling_mode: ScalingMode::WindowSize(60.0),
                ..default()
            }),
            ..default()
        },
        RaycastSource::<TileRaycastSet>::new_cursor(),
        InputManagerBundle::<CameraAction> {
            input_map: default_input_map_camera(),
            ..default()
        },
    ));
}

fn move_camera(
    time: Res<Time>,
    camera_focus: Query<&Transform, (With<CameraFocus>, Without<Camera>)>,
    mut camera: Query<(&mut Transform, &ActionState<CameraAction>), With<Camera>>,
) {
    let (mut camera_transform, action_state) = camera.single_mut();
    let delta = match camera_focus.get_single() {
        Ok(camera_focus) => {
            camera_focus.translation - camera_transform.translation + CAMERA_OFFSET_TO_PLAYER
        }
        Err(QuerySingleError::NoEntities(_)) => {
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
                dir.z -= 1.0;
            }
            if action_state.pressed(CameraAction::Down) {
                dir.z += 1.0;
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
            if dir.length() > 1.0 {
                if let Some(dir) = dir.try_normalize() {
                    dir * speed * time.delta_seconds()
                } else {
                    Vec3::ZERO
                }
            } else {
                dir * speed * time.delta_seconds()
            }
        }
        Err(QuerySingleError::MultipleEntities(e)) => {
            error!("Multiple Entities with CameraFocus component: {}", e);
            Vec3::ZERO
        }
    };

    camera_transform.translation += delta;
}

const MAX_ZOOM: f32 = 4.0;
const MIN_ZOOM: f32 = 0.5;

fn zoom_camera(mut query: Query<(&mut Projection, &ActionState<CameraAction>), With<Camera>>) {
    let (projection, action_state) = query.single_mut();

    let Projection::Orthographic(projection) = projection.into_inner() else {
        error!("Zooming isn't yet supported for perspective cameras.");
        return;
    };

    if let Some(direction) = zoom_direction(action_state, projection.scale) {
        projection.scale += 0.20 * direction;
    }
}

fn zoom_direction(action_state: &ActionState<CameraAction>, current_scaling: f32) -> Option<f32> {
    if action_state.pressed(CameraAction::ZoomOut) && current_scaling < MAX_ZOOM {
        Some(1.0)
    } else if action_state.pressed(CameraAction::ZoomIn) && current_scaling > MIN_ZOOM {
        Some(-1.0)
    } else {
        None
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
