use crate::load::CropId;
use bevy::prelude::Component;

#[derive(Component)]
pub struct ItemDrop {
    pub item_id: ItemId,
    pub amount: u16,
}

pub enum ItemId {
    Crop { crop_id: CropId },
}

impl ItemDrop {
    pub fn from_crop(crop_id: CropId, amount: u16) -> Self {
        Self {
            item_id: ItemId::Crop { crop_id },
            amount,
        }
    }
}
