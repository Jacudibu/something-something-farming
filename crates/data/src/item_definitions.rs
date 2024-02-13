use bevy::prelude::{Handle, Image, Resource, TextureAtlas};
use bevy::utils::HashMap;

use crate::item_id::CropId;

pub struct CropDefinition {
    pub id: CropId,
    pub name: String,
    pub stages: u8,
    pub growth_time_per_stage: u32,
    pub texture_atlas: Handle<TextureAtlas>,
    pub harvested_sprite: Handle<Image>,
}

#[derive(Resource)]
pub struct AllItems {
    pub crops: HashMap<CropId, CropDefinition>,
}
