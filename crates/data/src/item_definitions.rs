use bevy::prelude::{Handle, Image, Mesh, Resource, TextureAtlas};
use bevy::utils::HashMap;

use crate::prelude::{CropId, PropId};

/// An object which can be placed on tilled soil, and will grow over time.
pub struct CropDefinition {
    pub id: CropId,
    pub name: String,
    pub stages: u8,
    pub growth_time_per_stage: u32,
    pub texture_atlas: Handle<TextureAtlas>,
    pub harvested_sprite: Handle<Image>,
}

/// An object which can be placed in the world, and maybe further interacted with.
pub struct PropDefinition {
    pub id: PropId,
    pub name: String,
    pub mesh: Handle<Mesh>,
    pub texture: Handle<TextureAtlas>,
}

#[derive(Resource)]
pub struct AllItems {
    pub crops: HashMap<CropId, CropDefinition>,
    pub props: HashMap<PropId, PropDefinition>,
}
