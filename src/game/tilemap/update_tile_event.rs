use crate::game::tilemap::helpers::{below_of, left_of, right_of, top_of};
use crate::prelude::chunk_identifier::ChunkIdentifier;
use crate::prelude::helpers::determine_texture_index;
use crate::prelude::tilemap_layer::GroundLayer;
use crate::prelude::{ChunkPosition, WorldData};
use bevy::app::{App, Plugin, Update};
use bevy::prelude::{Event, EventReader, Query, Res, Without};
use bevy_ecs_tilemap::prelude::{TilePos, TileStorage, TileTextureIndex};

pub struct UpdateTileEventPlugin;
impl Plugin for UpdateTileEventPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<UpdateTileEvent>()
            .add_systems(Update, update_tiles);
    }
}

#[derive(Event)]
pub struct UpdateTileEvent {
    chunk_pos: ChunkPosition,
    tile_pos: TilePos,
}
impl UpdateTileEvent {
    pub fn new(chunk_pos: ChunkPosition, tile_pos: TilePos) -> Self {
        UpdateTileEvent {
            chunk_pos,
            tile_pos,
        }
    }

    pub fn surrounding_tiles(chunk_pos: ChunkPosition, tile_pos: TilePos) -> [Self; 4] {
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
    loaded_chunks: Query<(&TileStorage, &ChunkIdentifier), Without<GroundLayer>>,
    mut tiles: Query<&mut TileTextureIndex>,
) {
    for event in events.read() {
        let chunk = world_data.chunks.get(&event.chunk_pos).unwrap();
        let tile = chunk.at_pos(&event.tile_pos);

        if tile.is_tilled {
            if let Some((tile_storage, _)) = loaded_chunks
                .iter()
                .find(|(_, identifier)| identifier.position == event.chunk_pos)
            {
                let tile_entity = tile_storage.get(&event.tile_pos).unwrap();
                let mut index = tiles.get_mut(tile_entity).unwrap();
                *index = determine_texture_index(&event.tile_pos, &event.chunk_pos, &world_data);
            }
        }
    }
}
