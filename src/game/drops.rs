use crate::load::CropId;
use crate::GameState;
use bevy::app::{App, Plugin, Update};
use bevy::prelude::{
    in_state, Commands, Component, Entity, IntoSystemConfigs, Query, Res, Time, Transform, With,
    Without,
};

const PICKUP_DISTANCE: f32 = 5.0;
const MAGNET_DISTANCE: f32 = 50.0;
const MAGNET_SPEED: f32 = 150.0;

pub struct ItemPickupPlugin;
impl Plugin for ItemPickupPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            item_pickup_detection.run_if(in_state(GameState::Playing)),
        );
    }
}

#[derive(Component)]
pub struct ItemDrop {
    pub item_id: ItemId,
    pub amount: u16,
}

#[derive(Component)]
pub struct ItemMagnet {}

pub enum ItemId {
    Crop { crop_id: CropId },
}

fn item_pickup_detection(
    mut commands: Commands,
    mut drops: Query<(Entity, &ItemDrop, &mut Transform), Without<ItemMagnet>>,
    targets: Query<&Transform, With<ItemMagnet>>,
    time: Res<Time>,
) {
    for (entity, _drop, mut drop_transform) in drops.iter_mut() {
        let target = targets.iter().min_by(|a, b| {
            a.translation
                .truncate()
                .distance(drop_transform.translation.truncate())
                .total_cmp(
                    &b.translation
                        .truncate()
                        .distance(drop_transform.translation.truncate()),
                )
        });

        if let Some(target) = target {
            let delta = target.translation - drop_transform.translation;
            let distance = delta.truncate().length();
            if distance < PICKUP_DISTANCE {
                // TODO: PickupItemEvent
                commands.entity(entity).despawn();
            } else if distance < MAGNET_DISTANCE {
                let dir = delta.truncate().normalize();
                let movement = time.delta_seconds() * MAGNET_SPEED * dir;

                drop_transform.translation.x += movement.x;
                drop_transform.translation.y += movement.y;
            }
        }
    }
}

impl ItemDrop {
    pub fn from_crop(crop_id: CropId, amount: u16) -> Self {
        Self {
            item_id: ItemId::Crop { crop_id },
            amount,
        }
    }
}
