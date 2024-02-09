use bevy::app::{App, Plugin, Update};
use bevy::prelude::{in_state, IntoSystemConfigs, Query, Res, ResMut};
use bevy_sprite3d::AtlasSprite3dComponent;

use crate::prelude::loaded_chunks::LoadedChunks;
use crate::prelude::{AllCrops, MapPos, SimulationTime, WorldData};
use crate::GameState;

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
    mut sprites: Query<&mut AtlasSprite3dComponent>,
    simulation_time: Res<SimulationTime>,
    loaded_chunk_data: Res<LoadedChunks>,
    all_crops: Res<AllCrops>,
) {
    // TODO: Performance Improvements
    // Step #1: Cache the next item that should be updated instead of doing this every frame
    // Step #1.5: Mark cache as dirty when timers are changed once we have more things affecting this
    // Step #2: Collect all items which request an update in an ordered list, listen to events to add & remove them as needed

    if let Some(next) = find_next_tile_to_update(&world_data) {
        if next.update_at < simulation_time.elapsed_seconds_f32() {
            // TODO: Update
            let crop = world_data
                .chunks
                .get_mut(&next.pos.chunk)
                .unwrap()
                .crops
                .get_mut(&next.pos.tile)
                .unwrap();

            let crop_definition = all_crops.definitions.get(&crop.crop_id).unwrap();
            crop.stage += 1;
            if crop.stage < crop_definition.stages - 1 {
                crop.next_stage_at = Some(
                    simulation_time.elapsed_seconds_f32()
                        + crop_definition.growth_time_per_stage as f32,
                );
            } else {
                crop.next_stage_at = None;
            }

            // TODO: Move that into an event, so we can also play sound effects and animations when necessary
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
