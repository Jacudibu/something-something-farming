use crate::GameState;
use bevy::prelude::{App, Handle, Image, Plugin, Resource, TextureAtlas, Vec2};
use bevy::utils::HashMap;
use bevy_asset_loader::prelude::*;

pub struct LoadingPlugin;
impl Plugin for LoadingPlugin {
    fn build(&self, app: &mut App) {
        app.add_loading_state(
            LoadingState::new(GameState::Loading)
                .continue_to_state(GameState::Playing)
                .load_collection::<SpriteAssets>(),
        )
        .insert_resource(AllCrops::default());
    }
}

#[derive(Resource, AssetCollection)]
pub struct SpriteAssets {
    #[asset(texture_atlas(tile_size_x = 16.0, tile_size_y = 16.0, columns = 4, rows = 1))]
    #[asset(path = "sprites/debug_plant.png")]
    pub plant: Handle<TextureAtlas>,

    #[asset(path = "sprites/tile_cursor.png")]
    pub cursor: Handle<Image>,
    #[asset(path = "sprites/tilled_tile.png")]
    pub tilled_tiles: Handle<Image>,
    #[asset(path = "sprites/simple_tiles.png")]
    pub simple_tiles: Handle<Image>,
}

#[derive(Resource)]
pub struct AllCrops {
    pub(crate) definitions: HashMap<CropId, CropDefinition>,
}

impl Default for AllCrops {
    fn default() -> Self {
        let mut definitions = HashMap::new();

        definitions.insert(
            CropId(0),
            CropDefinition {
                id: CropId(0),
                stages: 4,
                growth_time_per_stage: 5,
            },
        );

        Self { definitions }
    }
}

pub struct CropDefinition {
    pub id: CropId,
    pub stages: u8,
    pub growth_time_per_stage: u32,
}

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct CropId(pub u32);
