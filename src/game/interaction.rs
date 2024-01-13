use crate::game::drops::{ItemDrop, ItemId};
use crate::game::map_pos::MapPos;
use crate::game::player::PlayerAction;
use crate::prelude::chunk_data::CropData;
use crate::prelude::helpers::determine_texture_index;
use crate::prelude::loaded_chunks::LoadedChunks;
use crate::prelude::tile_cursor::TileCursor;
use crate::prelude::tilemap_layer::GroundLayer;
use crate::prelude::update_tile_event::UpdateTileEvent;
use crate::prelude::{
    ActiveTool, CropId, MouseCursorOverUiState, SpriteAssets, WorldData, LAYER_CROPS,
    LAYER_ITEM_DROPS,
};
use crate::prelude::{AllCrops, GameState};
use bevy::prelude::*;
use bevy_ecs_tilemap::map::TilemapId;
use bevy_ecs_tilemap::prelude::TileBundle;
use bevy_ecs_tilemap::tiles::{TilePos, TileStorage};
use leafwing_input_manager::action_state::ActionState;
use std::ops::Deref;

pub struct InteractionPlugin;
impl Plugin for InteractionPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(ActiveTool::Hoe)
            .add_event::<CropDestroyedEvent>()
            .add_event::<CropHarvestedEvent>()
            .add_event::<TileInteractionEvent>()
            .add_systems(
                Update,
                select_active_tool.run_if(in_state(GameState::Playing)),
            )
            .add_systems(
                Update,
                detect_tile_interactions
                    .run_if(in_state(MouseCursorOverUiState::NotOverUI))
                    .run_if(in_state(GameState::Playing)),
            )
            .add_systems(
                Update,
                process_tile_interactions
                    .after(detect_tile_interactions)
                    .run_if(in_state(GameState::Playing)),
            )
            .add_systems(
                Update,
                process_delete_crops
                    .after(process_tile_interactions)
                    .run_if(in_state(GameState::Playing))
                    .run_if(on_event::<CropDestroyedEvent>()),
            )
            .add_systems(
                Update,
                process_harvested_crops
                    .after(process_tile_interactions)
                    .run_if(in_state(GameState::Playing))
                    .run_if(on_event::<CropHarvestedEvent>()),
            );
    }
}

fn select_active_tool(
    mut active_tool: ResMut<ActiveTool>,
    action_state: Query<&ActionState<PlayerAction>>,
) {
    let action_state = action_state.get_single();
    if action_state.is_err() {
        error!("PlayerAction State was missing!");
        return;
    }
    let action_state = action_state.unwrap();
    if action_state.just_pressed(PlayerAction::SelectHoe) {
        *active_tool = ActiveTool::Hoe;
    } else if action_state.just_pressed(PlayerAction::SelectPickaxe) {
        *active_tool = ActiveTool::Pickaxe;
    } else if action_state.just_pressed(PlayerAction::SelectSeed) {
        *active_tool = ActiveTool::Item {
            id: ItemId::Crop { crop_id: CropId(0) },
        };
    } else if action_state.just_pressed(PlayerAction::SelectScythe) {
        *active_tool = ActiveTool::Scythe;
    }
}

#[derive(Event, Debug)]
struct TileInteractionEvent {
    pub pos: MapPos,
    pub used_tool: ActiveTool,
}

#[derive(Event, Debug)]
struct CropDestroyedEvent {
    pub pos: MapPos,
}

#[derive(Event)]
struct CropHarvestedEvent {
    pub pos: MapPos,
    pub crop_id: CropId,
}

fn detect_tile_interactions(
    active_tool: Res<ActiveTool>,
    action_state: Query<&ActionState<PlayerAction>>,
    tile_cursor: Query<(&TileCursor, &Visibility)>,
    mut previously_interacted_tile: Local<Option<TilePos>>,
    mut tile_interaction_events: EventWriter<TileInteractionEvent>,
) {
    let action_state = action_state.get_single();
    if action_state.is_err() {
        error!("PlayerAction State was missing!");
        return;
    }
    let action_state = action_state.unwrap();

    if !action_state.pressed(PlayerAction::Interact) {
        return;
    }

    if action_state.just_pressed(PlayerAction::Interact) {
        *previously_interacted_tile = None;
    }

    for (cursor, visibility) in tile_cursor.iter() {
        if visibility == Visibility::Hidden {
            continue;
        }

        if let Some(previous) = *previously_interacted_tile {
            if previous == cursor.pos.tile {
                return;
            } else {
                *previously_interacted_tile = Some(cursor.pos.tile);
            }
        } else {
            *previously_interacted_tile = Some(cursor.pos.tile);
        }

        // in case we ever regularly happening AoE interaction events, it batch_send might be more performant
        tile_interaction_events.send(TileInteractionEvent {
            pos: cursor.pos.clone(),
            used_tool: active_tool.deref().clone(),
        });
    }
}

fn process_delete_crops(
    mut commands: Commands,
    mut world_data: ResMut<WorldData>,
    mut loaded_chunk_data: ResMut<LoadedChunks>,
    mut destroy_crop_events: EventReader<CropDestroyedEvent>,
) {
    for event in destroy_crop_events.read() {
        let chunk = world_data.chunks.get_mut(&event.pos.chunk).unwrap();

        if let Some(_) = chunk.crops.get(&event.pos.tile) {
            chunk.crops.remove(&event.pos.tile);

            if let Some(loaded_data) = loaded_chunk_data.chunks.get_mut(&event.pos.chunk) {
                if let Some(entity) = loaded_data.crops.remove(&event.pos.tile) {
                    commands.entity(entity).despawn();
                } else {
                    warn!("Prop was not set at {:?}.", event);
                }
            }
        }
    }
}

fn process_harvested_crops(
    mut commands: Commands,
    mut harvested_crop_events: EventReader<CropHarvestedEvent>,
    assets: Res<SpriteAssets>,
) {
    for event in harvested_crop_events.read() {
        // TODO: Consider bunching up nearby same-item drops into one bigger drop.
        // TODO: If chunk is not loaded, just add the item to whomever caused the interaction immediately if nearby
        // TODO: (premature) Drops should probably be persisted inside the chunk they're in and get (de-)spawned accordingly, otherwise 1000+ drops somewhere in the middle of nowhere might cause performance issues?
        // Also, if an NPC with Inventory walks through that chunk (maybe a bit further away from players than chunk loading distance so they won't notice as easily), they automagically pick it up?

        commands.spawn((
            Name::new("Drop"),
            SpriteBundle {
                transform: Transform::from_translation(event.pos.world_pos(LAYER_ITEM_DROPS)),
                texture: assets.debug_veggie.clone(),
                ..default()
            },
            ItemDrop::from_crop(event.crop_id, 1),
        ));
    }
}

fn process_tile_interactions(
    mut tile_interaction_event: EventReader<TileInteractionEvent>,
    mut commands: Commands,
    mut update_tile_events: EventWriter<UpdateTileEvent>,
    mut destroy_crop_events: EventWriter<CropDestroyedEvent>,
    mut harvest_crop_events: EventWriter<CropHarvestedEvent>,
    mut world_data: ResMut<WorldData>,
    mut object_chunks: Query<&mut TileStorage, Without<GroundLayer>>,
    mut loaded_chunk_data: ResMut<LoadedChunks>,
    time: Res<Time>,
    all_crops: Res<AllCrops>,
) {
    for event in tile_interaction_event.read() {
        match event.used_tool {
            ActiveTool::Hoe => {
                let world_data = &mut *world_data;
                let chunk = world_data.chunks.get_mut(&event.pos.chunk).unwrap();
                if chunk.at_pos(&event.pos.tile).is_tilled {
                    continue;
                }

                chunk.set_at_pos(&event.pos.tile, true);

                // TODO: Event - Place Floor tile
                if let Some(loaded_data) = loaded_chunk_data.chunks.get(&event.pos.chunk) {
                    let mut floor_tilemap_storage =
                        object_chunks.get_mut(loaded_data.floor_tilemap).unwrap();

                    let tilled_tile = commands
                        .spawn(TileBundle {
                            position: event.pos.tile.clone(),
                            tilemap_id: TilemapId(loaded_data.floor_tilemap),
                            texture_index: determine_texture_index(
                                &event.pos.tile,
                                &event.pos.chunk,
                                &world_data,
                            ),
                            ..Default::default()
                        })
                        .id();
                    commands
                        .entity(loaded_data.floor_tilemap)
                        .add_child(tilled_tile);
                    floor_tilemap_storage.set(&event.pos.tile, tilled_tile);
                    update_tile_events.send_batch(UpdateTileEvent::surrounding_tiles(
                        event.pos.chunk,
                        event.pos.tile,
                    ));
                }
            }
            ActiveTool::Pickaxe => {
                let chunk = world_data.chunks.get_mut(&event.pos.chunk).unwrap();
                if !chunk.at_pos(&event.pos.tile).is_tilled {
                    continue;
                }

                if let Some(_) = chunk.crops.get(&event.pos.tile) {
                    destroy_crop_events.send(CropDestroyedEvent { pos: event.pos });
                    continue;
                }

                // TODO: Event - Remove tilled tile
                chunk.set_at_pos(&event.pos.tile, false);
                if let Some(loaded_data) = loaded_chunk_data.chunks.get(&event.pos.chunk) {
                    let mut floor_tilemap_storage =
                        object_chunks.get_mut(loaded_data.floor_tilemap).unwrap();

                    if let Some(entity) = floor_tilemap_storage.get(&event.pos.tile) {
                        floor_tilemap_storage.remove(&event.pos.tile);
                        commands.entity(entity).despawn();
                    } else {
                        warn!("Entity was not set at {:?}.", event);
                    }

                    update_tile_events.send_batch(UpdateTileEvent::surrounding_tiles(
                        event.pos.chunk,
                        event.pos.tile,
                    ));
                }
            }
            ActiveTool::Scythe => {
                let chunk = world_data.chunks.get_mut(&event.pos.chunk).unwrap();

                if let Some(crop) = chunk.crops.get(&event.pos.tile) {
                    if crop.stage + 1 >= all_crops.definitions.get(&crop.crop_id).unwrap().stages {
                        harvest_crop_events.send(CropHarvestedEvent {
                            pos: event.pos,
                            crop_id: crop.crop_id,
                        });
                        destroy_crop_events.send(CropDestroyedEvent { pos: event.pos });
                    }
                }
            }
            ActiveTool::Item { id } => match id {
                ItemId::Crop { crop_id } => {
                    let chunk = world_data.chunks.get_mut(&event.pos.chunk).unwrap();
                    if !chunk.at_pos(&event.pos.tile).is_tilled {
                        continue;
                    }

                    if chunk.crops.get(&event.pos.tile).is_some() {
                        continue;
                    }

                    let crop_definition = all_crops.definitions.get(&crop_id).unwrap();
                    chunk
                        .crops
                        .insert(event.pos.tile, CropData::new(&crop_definition, &time));

                    // TODO: Event - Plant Seed
                    if let Some(loaded_data) = loaded_chunk_data.chunks.get_mut(&event.pos.chunk) {
                        let entity = commands
                            .spawn((
                                Name::new("Plant"),
                                SpriteSheetBundle {
                                    texture_atlas: crop_definition.texture_atlas.clone(),
                                    sprite: TextureAtlasSprite::new(0),
                                    transform: Transform::from_translation(
                                        event.pos.pos_inside_chunk(LAYER_CROPS),
                                    ),
                                    ..default()
                                },
                            ))
                            .set_parent(loaded_data.ground_tilemap)
                            .id();

                        loaded_data.crops.insert(event.pos.tile, entity);
                    }
                }
            },
        }
    }
}
