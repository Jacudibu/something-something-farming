use crate::game::player::PlayerAction;
use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use bevy_egui::{egui, EguiContext};
use leafwing_input_manager::action_state::ActionState;

pub struct DebugOverlayPlugin;
impl Plugin for DebugOverlayPlugin {
    fn build(&self, app: &mut App) {
        app.add_state::<DebugOverlayState>()
            .add_plugins(bevy_inspector_egui::DefaultInspectorConfigPlugin)
            .add_systems(Update, trigger_debug_overlay)
            .add_systems(
                Update,
                world_inspector_ui.run_if(in_state(DebugOverlayState::On)),
            );
    }
}

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
pub enum DebugOverlayState {
    #[default]
    Off,
    On,
}

fn trigger_debug_overlay(
    input_state: Query<&ActionState<PlayerAction>>,
    current_state: Res<State<DebugOverlayState>>,
    mut next_state: ResMut<NextState<DebugOverlayState>>,
) {
    if let Ok(state) = input_state.get_single() {
        if state.just_pressed(PlayerAction::ToggleDebugOverlay) {
            match current_state.get() {
                DebugOverlayState::Off => next_state.set(DebugOverlayState::On),
                DebugOverlayState::On => next_state.set(DebugOverlayState::Off),
            }
        }
    }
}

fn world_inspector_ui(world: &mut World) {
    let Ok(egui_context) = world
        .query_filtered::<&mut EguiContext, With<PrimaryWindow>>()
        .get_single(world)
    else {
        return;
    };
    let mut egui_context = egui_context.clone();

    egui::Window::new("World Inspector").show(egui_context.get_mut(), |ui| {
        egui::ScrollArea::vertical().show(ui, |ui| {
            // equivalent to `WorldInspectorPlugin`
            bevy_inspector_egui::bevy_inspector::ui_for_world(world, ui);

            egui::CollapsingHeader::new("Materials").show(ui, |ui| {
                bevy_inspector_egui::bevy_inspector::ui_for_assets::<StandardMaterial>(world, ui);
            });

            // ui.heading("Entities");
            // bevy_inspector_egui::bevy_inspector::ui_for_world_entities(world, ui);
        });
    });
}
