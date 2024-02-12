use bevy::app::App;
use bevy::prelude::{
    in_state, Commands, DespawnRecursiveExt, Entity, IntoSystemConfigs, Local, Plugin, Query, Res,
    Update,
};

use crate::game::walls::build_and_spawn_wall_entity;
use crate::load::{DebugMaterials, DebugMeshes};
use crate::prelude::loaded_chunks::LoadedChunks;
use crate::prelude::tile_cursor::TileCursor;
use crate::prelude::{ActiveTool, CardinalDirection, MapPos};
use crate::GameState;

pub struct InteractionPreviewPlugin;
impl Plugin for InteractionPreviewPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, update_preview.run_if(in_state(GameState::Playing)));
    }
}

struct LastFramePreviewDataCell {
    pos: MapPos,
    tile_edge: CardinalDirection,
    tool: ActiveTool,
    preview_entity: Entity,
}

#[derive(Default)]
struct LastFramePreviewData {
    previews: Vec<LastFramePreviewDataCell>,
}

fn update_preview(
    mut commands: Commands,
    loaded_chunk_data: Res<LoadedChunks>,
    active_tool: Res<ActiveTool>,
    cursor_query: Query<&TileCursor>,
    debug_materials: Res<DebugMaterials>,
    debug_meshes: Res<DebugMeshes>,
    mut last_frame_preview_data: Local<LastFramePreviewData>,
) {
    // TODO: don't just despawn & clear
    for x in &last_frame_preview_data.previews {
        commands.entity(x.preview_entity).despawn_recursive();
    }

    last_frame_preview_data.previews.clear();

    for cursor in cursor_query.iter() {
        // TODO: Skip unchanged previews

        let tile = {
            if let Some(loaded_data) = loaded_chunk_data.chunks.get(&cursor.pos.chunk) {
                if let Some(tile) = loaded_data.get_tile(cursor.pos.tile.x, cursor.pos.tile.y) {
                    tile
                } else {
                    continue;
                }
            } else {
                continue;
            }
        };

        match *active_tool {
            ActiveTool::None => {}
            ActiveTool::Item(_) => {}
            ActiveTool::Wall => {
                let entity = build_and_spawn_wall_entity(
                    &mut commands,
                    tile,
                    cursor.tile_edge,
                    &debug_meshes,
                    &debug_materials,
                );

                last_frame_preview_data
                    .previews
                    .push(LastFramePreviewDataCell {
                        pos: cursor.pos,
                        tile_edge: cursor.tile_edge,
                        tool: active_tool.clone(),
                        preview_entity: entity,
                    });
            }
        }
    }

    // TODO: Delete previews that no longer exist here
}
