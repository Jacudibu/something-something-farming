use crate::game::item_id::{CropId, ItemId};
use crate::prelude::Inventory;
use crate::GameState;
use bevy::log::error;
use bevy::prelude::{
    in_state, on_event, App, Commands, Component, Entity, Event, EventReader, EventWriter,
    IntoSystemConfigs, Plugin, Query, Res, Time, Transform, Update, Without,
};

const PICKUP_DISTANCE: f32 = 5.0;
const DEFAULT_MAGNET_DISTANCE: f32 = 40.0;
const DEFAULT_MAGNET_SPEED: f32 = 200.0;

pub struct ItemPickupPlugin;
impl Plugin for ItemPickupPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<PickupItemDropEvent>().add_systems(
            Update,
            (
                item_magnet_and_pickups.run_if(in_state(GameState::Playing)),
                add_item_pickups_to_inventory.run_if(on_event::<PickupItemDropEvent>()),
            ),
        );
    }
}

#[derive(Component, Clone)]
pub struct ItemDrop {
    pub item_id: ItemId,
    pub amount: u32,
}

#[derive(Component)]
pub struct ItemMagnet {
    distance: f32,
    speed: f32,
}

impl Default for ItemMagnet {
    fn default() -> Self {
        Self {
            distance: DEFAULT_MAGNET_DISTANCE,
            speed: DEFAULT_MAGNET_SPEED,
        }
    }
}

#[derive(Event)]
pub struct PickupItemDropEvent {
    drop: ItemDrop,
    entity: Entity,
}

fn add_item_pickups_to_inventory(
    mut events: EventReader<PickupItemDropEvent>,
    mut entities_with_inventory: Query<&mut Inventory>,
) {
    for event in events.read() {
        if let Ok(mut inventory) = entities_with_inventory.get_mut(event.entity) {
            inventory.add_item(&event.drop.item_id, event.drop.amount);
        } else {
            error!(
                "Pickup Item event for entity without an inventory: {:?}",
                event.entity
            )
        }
    }
}

fn item_magnet_and_pickups(
    mut commands: Commands,
    mut drops: Query<(Entity, &ItemDrop, &mut Transform), Without<ItemMagnet>>,
    mut pickup_events: EventWriter<PickupItemDropEvent>,
    targets: Query<(Entity, &Transform, &ItemMagnet)>,
    time: Res<Time>,
) {
    for (entity, drop, mut drop_transform) in drops.iter_mut() {
        let target = targets.iter().min_by(|a, b| {
            a.1.translation
                .truncate()
                .distance(drop_transform.translation.truncate())
                .total_cmp(
                    &b.1.translation
                        .truncate()
                        .distance(drop_transform.translation.truncate()),
                )
        });

        if let Some((target_entity, target_transform, magnet)) = target {
            let delta = target_transform.translation - drop_transform.translation;
            let distance = delta.truncate().length();
            if distance < PICKUP_DISTANCE {
                pickup_events.send(PickupItemDropEvent {
                    drop: drop.clone(),
                    entity: target_entity,
                });
                commands.entity(entity).despawn();
            } else if distance < magnet.distance {
                let dir = delta.truncate().normalize();
                let speed = {
                    let percentage = (magnet.distance - distance) / magnet.distance;
                    let speed = magnet.speed * percentage.powf(1.5);
                    if speed < magnet.speed * 0.1 {
                        magnet.speed * 0.1
                    } else {
                        speed
                    }
                };
                let movement = time.delta_seconds() * speed * dir;

                drop_transform.translation.x += movement.x;
                drop_transform.translation.y += movement.y;
            }
        }
    }
}

impl ItemDrop {
    pub fn from_crop(crop_id: CropId, amount: u32) -> Self {
        Self {
            item_id: ItemId::Crop { crop_id },
            amount,
        }
    }
}
