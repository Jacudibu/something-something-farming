use crate::prelude::loaded_chunks::LoadedChunks;
use crate::prelude::{MapPos, WorldData};
use crate::GameState;
use bevy::app::{App, Plugin, Update};
use bevy::prelude::{in_state, IntoSystemConfigs, Query, Res, ResMut, TextureAtlasSprite, Time};

pub struct TileUpdaterPlugin;
impl Plugin for TileUpdaterPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, update_tiles.run_if(in_state(GameState::Playing)));
    }
}

struct NextItemToUpdate {
    update_at: f32,
    pos: MapPos,
}

fn update_tiles(
    mut world_data: ResMut<WorldData>,
    mut sprites: Query<&mut TextureAtlasSprite>,
    time: Res<Time>,
    loaded_chunk_data: Res<LoadedChunks>,
) {
    // TODO: Performance Improvements
    // Step #1: Cache the next item that should be updated instead of doing this every frame
    // Step #1.5: Mark cache as dirty when timers are changed once we have more things affecting this
    // Step #2: Collect all items which request an update in an ordered list, listen to events to add & remove them as needed

    if let Some(next) = find_next_tile_to_update(&world_data) {
        if next.update_at < time.elapsed_seconds() {
            // TODO: Update
            let crop = world_data
                .chunks
                .get_mut(&next.pos.chunk)
                .unwrap()
                .crops
                .get_mut(&next.pos.tile)
                .unwrap();

            crop.stage += 1;
            // TODO: Move those values into a config file
            if crop.stage < 3 {
                crop.next_stage_at = Some(time.elapsed_seconds() + 5.0);
            } else {
                crop.next_stage_at = None;
            }

            // TODO: Move that into an event
            if let Some(chunk) = loaded_chunk_data.chunks.get(&next.pos.chunk) {
                if let Some(entity) = chunk.crops.get(&next.pos.tile) {
                    if let Ok(mut bla) = sprites.get_mut(entity.clone()) {
                        bla.index += 1;
                    }
                }
            }
        }
    }
}

fn find_next_tile_to_update(world_data: &WorldData) -> Option<NextItemToUpdate> {
    let mut lowest_time_found = f32::MAX;
    let mut next: Option<NextItemToUpdate> = None;
    for (chunk_pos, chunk) in world_data.chunks.iter() {
        for (tile_pos, crop) in chunk.crops.iter() {
            if let Some(remaining_time) = crop.next_stage_at {
                if lowest_time_found > remaining_time {
                    lowest_time_found = remaining_time;
                    next = Some(NextItemToUpdate {
                        update_at: remaining_time,
                        pos: MapPos::new(chunk_pos.clone(), tile_pos.clone()),
                    })
                }
            }
        }
    }

    next
}