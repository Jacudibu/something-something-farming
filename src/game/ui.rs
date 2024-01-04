use crate::prelude::tile_cursor::TileCursor;
use crate::prelude::WorldData;
use bevy::app::{App, Plugin, Update};
use bevy::prelude::{error, Query, Res};
use bevy_egui::egui::Pos2;
use bevy_egui::{egui, EguiContexts, EguiPlugin};

pub struct UiPlugin;
impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(EguiPlugin).add_systems(Update, ui_system);
    }
}

fn ui_system(mut contexts: EguiContexts, cursor: Query<&TileCursor>, world_data: Res<WorldData>) {
    let cursor = cursor
        .get_single()
        .expect("Multiselection isn't yet supported by debug ui");

    let chunk = world_data.chunks.get(&cursor.chunk_pos);
    if chunk.is_none() {
        error!("Chunk at {} did not exist!", cursor.chunk_pos);
        return;
    }
    let chunk = chunk.unwrap();
    let tile = chunk.at_pos(&cursor.tile_pos);

    egui::Window::new(format!("{}", cursor.global_position()))
        .collapsible(false)
        .resizable(false)
        .fixed_pos(Pos2::new(5.0, 5.0))
        .show(contexts.ctx_mut(), |ui| {
            ui.label(format!("Chunk: {}", cursor.chunk_pos));
            ui.label(format!(
                "Local Position: [{}, {}]",
                cursor.tile_pos.x, cursor.tile_pos.y
            ));
            ui.label(format!("Tile: {:?}", tile))
        });
}
