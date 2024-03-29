use bevy::app::App;
use bevy::prelude::{
    in_state, Commands, DespawnRecursiveExt, Entity, IntoSystemConfigs, Local, Plugin, Query, Res,
    Update,
};
use bevy::utils::HashMap;

use crate::game::walls::build_and_spawn_wall_entity_with_mesh_and_material;
use crate::load::{DebugMaterials, DebugMeshes};
use crate::prelude::interaction::BuildingRotation;
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
    rotation: CardinalDirection,
    tool: ActiveTool,
    preview_entity: Entity,
}

#[derive(Default)]
struct LastFramePreviewData {
    previews: HashMap<MapPos, LastFramePreviewDataCell>,
}

fn update_preview(
    mut commands: Commands,
    loaded_chunk_data: Res<LoadedChunks>,
    active_tool: Res<ActiveTool>,
    building_rotation: Res<BuildingRotation>,
    cursor_query: Query<&TileCursor>,
    debug_materials: Res<DebugMaterials>,
    debug_meshes: Res<DebugMeshes>,
    mut last_frame_preview_data: Local<LastFramePreviewData>,
) {
    let mut still_highlighted_positions = Vec::new();
    for cursor in cursor_query.iter() {
        still_highlighted_positions.push(&cursor.pos);
        if let Some(existing_preview) = last_frame_preview_data.previews.get(&cursor.pos) {
            if existing_preview.tool == *active_tool
                && existing_preview.rotation == building_rotation.direction
            {
                continue;
            } else {
                commands
                    .entity(existing_preview.preview_entity)
                    .despawn_recursive();
                last_frame_preview_data.previews.remove(&cursor.pos);
            }
        }

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
                let entity = build_and_spawn_wall_entity_with_mesh_and_material(
                    &mut commands,
                    tile,
                    building_rotation.direction,
                    debug_meshes.wall.clone(),
                    debug_materials.preview_ghost.clone(),
                );

                last_frame_preview_data.previews.insert(
                    cursor.pos,
                    LastFramePreviewDataCell {
                        rotation: building_rotation.direction,
                        tool: active_tool.clone(),
                        preview_entity: entity,
                    },
                );
            }
        }
    }

    for (_pos, obsolete_cell) in last_frame_preview_data
        .previews
        .extract_if(|pos, _value| !still_highlighted_positions.contains(&pos))
    {
        commands
            .entity(obsolete_cell.preview_entity)
            .despawn_recursive();
    }
}
