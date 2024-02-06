use crate::game::drops::ItemMagnet;
use crate::load::SpriteAssets;
use crate::prelude::camera::CameraFocus;
use crate::prelude::{Inventory, LAYER_PLAYER, SPRITE_DEFAULT_PIVOT, SPRITE_PIXELS_PER_METER};
use crate::GameState;
use bevy::app::{App, Plugin, Update};
use bevy::core::Name;
use bevy::math::Vec3;
use bevy::pbr::AlphaMode;
use bevy::prelude::{
    default, in_state, Commands, Component, GamepadButtonType, IntoSystemConfigs, KeyCode,
    MouseButton, OnEnter, Query, Reflect, Res, Time, Transform, With,
};
use bevy_sprite3d::{Sprite3d, Sprite3dParams};
use leafwing_input_manager::action_state::ActionState;
use leafwing_input_manager::axislike::{DeadZoneShape, DualAxis};
use leafwing_input_manager::input_map::InputMap;
use leafwing_input_manager::prelude::UserInput;
use leafwing_input_manager::user_input::InputKind;
use leafwing_input_manager::{Actionlike, InputManagerBundle};

pub struct PlayerPlugin;
impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Playing), initialize_player);
        app.add_systems(Update, move_player.run_if(in_state(GameState::Playing)));
    }
}

#[derive(Component)]
pub struct ControlledByPlayer {}

const PLAYER_SPEED: f32 = 100.0;

fn initialize_player(
    mut commands: Commands,
    mut sprite_params: Sprite3dParams,
    assets: Res<SpriteAssets>,
) {
    commands.spawn((
        Name::new("Player"),
        Sprite3d {
            image: assets.debug_character.clone(),
            transform: Transform::from_xyz(0.0, 0.0, 0.0),
            pivot: SPRITE_DEFAULT_PIVOT,
            alpha_mode: AlphaMode::Blend,
            pixels_per_metre: SPRITE_PIXELS_PER_METER,
            ..default()
        }
        .bundle(&mut sprite_params),
        ControlledByPlayer {},
        InputManagerBundle::<PlayerAction> {
            input_map: default_input_map(),
            ..default()
        },
        CameraFocus {},
        ItemMagnet::default(),
        Inventory::default(),
    ));
}

fn move_player(
    time: Res<Time>,
    mut query: Query<(&mut Transform, &ActionState<PlayerAction>), With<ControlledByPlayer>>,
) {
    let (mut transform, action_state) = query.single_mut();
    let mut dir;
    if action_state.pressed(PlayerAction::Move) {
        dir = action_state
            .clamped_axis_pair(PlayerAction::Move)
            .unwrap()
            .xy()
            .extend(0.0);
    } else {
        dir = Vec3::ZERO;
    }

    if action_state.pressed(PlayerAction::Up) {
        dir.y += 1.0;
    }
    if action_state.pressed(PlayerAction::Down) {
        dir.y -= 1.0;
    }
    if action_state.pressed(PlayerAction::Right) {
        dir.x += 1.0;
    }
    if action_state.pressed(PlayerAction::Left) {
        dir.x -= 1.0;
    }

    let speed = PLAYER_SPEED;
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
}

#[derive(Actionlike, PartialEq, Eq, Hash, Clone, Copy, Debug, Reflect)]
pub enum PlayerAction {
    Move,
    Up,
    Down,
    Left,
    Right,
    Interact,
    Hotbar1,
    Hotbar2,
    Hotbar3,
    Hotbar4,
    Hotbar5,
    ToggleDebugOverlay,
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

    input_map.insert(KeyCode::Key1, PlayerAction::Hotbar1);
    input_map.insert(KeyCode::Key2, PlayerAction::Hotbar2);
    input_map.insert(KeyCode::Key3, PlayerAction::Hotbar3);
    input_map.insert(KeyCode::Key4, PlayerAction::Hotbar4);
    input_map.insert(KeyCode::Key5, PlayerAction::Hotbar5);
    input_map.insert(KeyCode::F2, PlayerAction::ToggleDebugOverlay);

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
