use crate::prelude::tile_cursor::TileCursor;
use crate::prelude::{ActiveTool, GameState, WorldData};
use bevy::app::{App, First, Plugin, Update};
use bevy::log::error;
use bevy::prelude::{in_state, IntoSystemConfigs, NextState, Query, Res, ResMut, State, States};
use bevy_egui::egui::{Align2, Pos2};
use bevy_egui::{egui, EguiContexts, EguiPlugin};

pub struct UiPlugin;
impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        if !app.is_plugin_added::<EguiPlugin>() {
            app.add_plugins(EguiPlugin);
        }

        app.add_state::<MouseCursorOverUiState>()
            .add_systems(First, detect_mouse_cursor_over_ui)
            .add_systems(Update, ui_system.run_if(in_state(GameState::Playing)));
    }
}

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
pub enum MouseCursorOverUiState {
    #[default]
    NotOverUI,
    OverUI,
}

fn detect_mouse_cursor_over_ui(
    mut contexts: EguiContexts,
    current_mouse_state: Res<State<MouseCursorOverUiState>>,
    mut next_state: ResMut<NextState<MouseCursorOverUiState>>,
) {
    if contexts.ctx_mut().is_pointer_over_area() {
        if current_mouse_state.get() != &MouseCursorOverUiState::OverUI {
            next_state.set(MouseCursorOverUiState::OverUI);
        }
    } else {
        if current_mouse_state.get() != &MouseCursorOverUiState::NotOverUI {
            next_state.set(MouseCursorOverUiState::NotOverUI);
        }
    }
}

fn ui_system(
    mut contexts: EguiContexts,
    cursor: Query<&TileCursor>,
    world_data: Res<WorldData>,
    active_tool: Res<ActiveTool>,
) {
    let cursor = cursor
        .get_single()
        .expect("Multiselection isn't yet supported by debug ui");

    let chunk = world_data.chunks.get(&cursor.pos.chunk);
    if chunk.is_none() {
        error!("Chunk at {} did not exist!", cursor.pos.chunk);
        return;
    }
    let chunk = chunk.unwrap();
    let tile = chunk.at_pos(&cursor.pos.tile);

    egui::Window::new(format!("{}", cursor.global_position()))
        .collapsible(false)
        .resizable(false)
        .fixed_pos(Pos2::new(5.0, 5.0))
        .show(contexts.ctx_mut(), |ui| {
            ui.label(format!("Chunk: {}", cursor.pos.chunk));
            ui.label(format!(
                "Local Position: [{}, {}]",
                cursor.pos.tile.x, cursor.pos.tile.y
            ));
            ui.label(format!("Tile: {:?}", tile))
        });

    egui::Window::new("Active Tool View")
        .title_bar(false)
        .collapsible(false)
        .resizable(false)
        .anchor(Align2::LEFT_BOTTOM, egui::Vec2::new(0.0, 0.0))
        .fixed_pos(Pos2::new(5.0, 5.0))
        .show(contexts.ctx_mut(), |ui| ui.label(active_tool.to_string()));
}
