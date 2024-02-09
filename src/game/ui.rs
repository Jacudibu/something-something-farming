use bevy::app::{App, First, Plugin, Update};
use bevy::log::error;
use bevy::prelude::{
    in_state, IntoSystemConfigs, Name, NextState, Query, Res, ResMut, State, States,
};
use bevy_egui::egui::{Align2, Pos2};
use bevy_egui::{egui, EguiContexts, EguiPlugin};

use crate::load::AllCrops;
use crate::prelude::chunk_data::ChunkData;
use crate::prelude::tile_cursor::TileCursor;
use crate::prelude::{ActiveTool, GameState, Inventory, MapPos, SimulationTime, WorldData};

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
    simulation_time: Res<SimulationTime>,
    all_crops: Res<AllCrops>,
    inventories: Query<(&Name, &Inventory)>,
) {
    if let Ok(cursor) = cursor.get_single() {
        let chunk = world_data.chunks.get(&cursor.pos.chunk);
        if chunk.is_none() {
            error!("Chunk at {} did not exist!", cursor.pos.chunk);
            return;
        }
        let chunk = chunk.unwrap();
        egui::Window::new(format!("{}", cursor.global_position()))
            .collapsible(false)
            .resizable(false)
            .fixed_pos(Pos2::new(5.0, 5.0))
            .show(contexts.ctx_mut(), |ui| {
                ui.label(map_data_for_position(
                    chunk,
                    &cursor.pos,
                    &simulation_time,
                    &all_crops,
                ));
            });
    }

    egui::Window::new("Active Tool View")
        .title_bar(false)
        .collapsible(false)
        .resizable(false)
        .anchor(Align2::LEFT_BOTTOM, egui::Vec2::new(0.0, 0.0))
        .fixed_pos(Pos2::new(5.0, 5.0))
        .show(contexts.ctx_mut(), |ui| ui.label(active_tool.to_string()));

    if !inventories.is_empty() {
        egui::Window::new("Inventories")
            .title_bar(true)
            .collapsible(true)
            .resizable(false)
            .anchor(Align2::RIGHT_TOP, egui::Vec2::new(0.0, 0.0))
            .fixed_pos(Pos2::new(5.0, 5.0))
            .show(contexts.ctx_mut(), |ui| {
                for (name, inventory) in inventories.iter() {
                    ui.collapsing(name.to_string(), |content| {
                        if inventory.is_empty() {
                            content.label("Empty!");
                        } else {
                            for (id, amount) in inventory.into_iter() {
                                content.label(format!("{}: {}", id.item_name(&all_crops), amount));
                            }
                        }
                    });
                }
            });
    }
}

fn map_data_for_position(
    chunk: &ChunkData,
    pos: &MapPos,
    simulation_time: &SimulationTime,
    all_crops: &AllCrops,
) -> String {
    let mut lines = Vec::new();
    lines.push(format!("Chunk: {}", pos.chunk));
    lines.push(format!("Local Position: [{}, {}]", pos.tile.x, pos.tile.y));

    let tile = chunk.at_pos(&pos.tile);

    lines.push(format!(
        "Tile: {:?}\n  is_tilled: {}",
        tile.ground_type, tile.is_tilled
    ));

    if let Some(crop) = chunk.crops.get(&pos.tile) {
        let definition = all_crops.definitions.get(&crop.crop_id).unwrap();
        lines.push(format!("Crop: {} ({})", definition.name, crop.crop_id.0));
        lines.push(format!("  stage: {}/{}", crop.stage + 1, definition.stages));
        if let Some(next_stage) = crop.next_stage_at {
            lines.push(format!(
                "  next: {:.1}",
                next_stage - simulation_time.elapsed_seconds_f32()
            ));
        }
    }

    lines.join("\n")
}
