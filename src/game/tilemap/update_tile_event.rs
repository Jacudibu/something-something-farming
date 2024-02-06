use crate::game::tilemap::helpers::{below_of, left_of, right_of, top_of};
use crate::load::DebugMaterials;
use crate::prelude::chunk_identifier::ChunkIdentifier;
use crate::prelude::helpers::determine_texture_index;
use crate::prelude::loaded_chunks::LoadedChunks;
use crate::prelude::tilemap_layer::GroundLayer;
use crate::prelude::GameState;
use crate::prelude::{ChunkPos, WorldData};
use bevy::app::{App, Plugin, Update};
use bevy::prelude::{
    in_state, Event, EventReader, Handle, IntoSystemConfigs, Mesh, Query, Res, StandardMaterial,
    With, Without,
};
use bevy_ecs_tilemap::prelude::{TilePos, TileStorage, TileTextureIndex};

pub struct UpdateTileEventPlugin;
impl Plugin for UpdateTileEventPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<UpdateTileEvent>()
            .add_systems(Update, update_tiles.run_if(in_state(GameState::Playing)));
    }
}

#[derive(Event)]
pub struct UpdateTileEvent {
    chunk_pos: ChunkPos,
    tile_pos: TilePos,
}
impl UpdateTileEvent {
    pub fn new(chunk_pos: ChunkPos, tile_pos: TilePos) -> Self {
        UpdateTileEvent {
            chunk_pos,
            tile_pos,
        }
    }

    pub fn surrounding_tiles(chunk_pos: ChunkPos, tile_pos: TilePos) -> [Self; 4] {
        let left = left_of(&chunk_pos, &tile_pos);
        let right = right_of(&chunk_pos, &tile_pos);
        let top = top_of(&chunk_pos, &tile_pos);
        let bottom = below_of(&chunk_pos, &tile_pos);
        [
            UpdateTileEvent::new(left.0, left.1),
            UpdateTileEvent::new(right.0, right.1),
            UpdateTileEvent::new(top.0, top.1),
            UpdateTileEvent::new(bottom.0, bottom.1),
        ]
    }
}

fn update_tiles(
    mut events: EventReader<UpdateTileEvent>,
    world_data: Res<WorldData>,
    loaded_chunks: Res<LoadedChunks>,
    mut tiles: Query<&mut Handle<StandardMaterial>, With<Handle<Mesh>>>,
    debug_materials: Res<DebugMaterials>,
) {
    for event in events.read() {
        if let Some(chunk) = world_data.chunks.get(&event.chunk_pos) {
            let tile = chunk.at_pos(&event.tile_pos);

            if let Some(loaded_chunk_data) = loaded_chunks.chunks.get(&event.chunk_pos) {
                let tile_entity = loaded_chunk_data
                    .get_tile(event.tile_pos.x, event.tile_pos.y)
                    .unwrap();
                let mut material = tiles.get_mut(tile_entity).unwrap();

                if tile.is_tilled {
                    // FIXME: determine which texture we wanna use, maybe use a TextureAtlas while we are at it
                    // determine_texture_index(&event.tile_pos, &event.chunk_pos, &world_data);
                    *material = debug_materials.single_tile_tilled.clone();
                } else {
                    *material = debug_materials.single_tile.clone();
                }
            }
        }
    }
}
