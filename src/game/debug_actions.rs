use bevy::app::{App, Update};
use bevy::ecs::system::SystemParam;
use bevy::log::info;
use bevy::prelude::{KeyCode, NextState, Plugin, Reflect, Res, ResMut, State, States};
use leafwing_input_manager::action_state::ActionState;
use leafwing_input_manager::input_map::InputMap;
use leafwing_input_manager::plugin::InputManagerPlugin;
use leafwing_input_manager::Actionlike;

pub struct DebugActionPlugin;
impl Plugin for DebugActionPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(InputManagerPlugin::<DebugAction>::default())
            .init_resource::<ActionState<DebugAction>>()
            .insert_resource(create_input_map())
            .add_systems(Update, track_input)
            .add_state::<DebugOverlayState>()
            .add_state::<DebugWallVisibilityState>();
    }
}

#[derive(Actionlike, PartialEq, Eq, Hash, Clone, Copy, Debug, Reflect)]
enum DebugAction {
    ToggleWallVisibility,
    ToggleDebugOverlay,
}

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
pub enum DebugOverlayState {
    #[default]
    Off,
    On,
}

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
pub enum DebugWallVisibilityState {
    #[default]
    Visible,
    Hidden,
}

#[derive(SystemParam)]
struct CurrentStates<'w> {
    debug_overlay: Res<'w, State<DebugOverlayState>>,
    wall_visibility: Res<'w, State<DebugWallVisibilityState>>,
}

#[derive(SystemParam)]
struct StateChanges<'w> {
    debug_overlay: ResMut<'w, NextState<DebugOverlayState>>,
    wall_visibility: ResMut<'w, NextState<DebugWallVisibilityState>>,
}

fn track_input(
    input_state: Res<ActionState<DebugAction>>,
    current_states: CurrentStates,
    mut state_changes: StateChanges,
) {
    if input_state.just_pressed(DebugAction::ToggleDebugOverlay) {
        match current_states.debug_overlay.get() {
            DebugOverlayState::Off => state_changes.debug_overlay.set(DebugOverlayState::On),
            DebugOverlayState::On => state_changes.debug_overlay.set(DebugOverlayState::Off),
        }
    }
    if input_state.just_pressed(DebugAction::ToggleWallVisibility) {
        match current_states.wall_visibility.get() {
            DebugWallVisibilityState::Hidden => state_changes
                .wall_visibility
                .set(DebugWallVisibilityState::Visible),
            DebugWallVisibilityState::Visible => state_changes
                .wall_visibility
                .set(DebugWallVisibilityState::Hidden),
        }
    }
}

fn create_input_map() -> InputMap<DebugAction> {
    let mut input_map = InputMap::default();

    input_map.insert(KeyCode::F1, DebugAction::ToggleWallVisibility);
    input_map.insert(KeyCode::F2, DebugAction::ToggleDebugOverlay);

    input_map
}
