use crate::GameState;
use bevy::prelude::*;
use bevy::utils::HashMap;
use bevy_asset_loader::prelude::*;

pub struct LoadingPlugin;
impl Plugin for LoadingPlugin {
    fn build(&self, app: &mut App) {
        app.add_loading_state(
            LoadingState::new(GameState::Loading)
                .continue_to_state(GameState::Playing)
                .load_collection::<SpriteAssets>()
                .load_collection::<HardcodedCropAssetsThatShouldBeTurnedIntoDynamicResourcesEventually>(),
        )
            .add_systems(OnExit(GameState::Loading), insert_crop_resource);
    }
}

#[derive(Resource, AssetCollection)]
pub struct SpriteAssets {
    #[asset(path = "sprites/tile_cursor.png")]
    pub cursor: Handle<Image>,
    #[asset(path = "sprites/tilled_tile.png")]
    pub tilled_tiles: Handle<Image>,
    #[asset(path = "sprites/simple_tiles.png")]
    pub simple_tiles: Handle<Image>,
    #[asset(path = "sprites/debug_character.png")]
    pub debug_character: Handle<Image>,
}

#[derive(Resource, AssetCollection)]
pub struct HardcodedCropAssetsThatShouldBeTurnedIntoDynamicResourcesEventually {
    #[asset(texture_atlas(tile_size_x = 16.0, tile_size_y = 16.0, columns = 4, rows = 1))]
    #[asset(path = "sprites/blue_debug_plant.png")]
    pub blue_debug_plant: Handle<TextureAtlas>,
    #[asset(path = "sprites/blue_debug_veggie.png")]
    pub blue_debug_veggie: Handle<Image>,

    #[asset(texture_atlas(tile_size_x = 16.0, tile_size_y = 16.0, columns = 4, rows = 1))]
    #[asset(path = "sprites/red_debug_plant.png")]
    pub red_debug_plant: Handle<TextureAtlas>,
    #[asset(path = "sprites/red_debug_veggie.png")]
    pub red_debug_veggie: Handle<Image>,
}

fn insert_crop_resource(world: &mut World) {
    let assets = world
        .get_resource::<HardcodedCropAssetsThatShouldBeTurnedIntoDynamicResourcesEventually>()
        .expect("Hardcoded assets should be loaded! :(");
    world.insert_resource(AllCrops::from(&assets));
}

#[derive(Resource)]
pub struct AllCrops {
    pub(crate) definitions: HashMap<CropId, CropDefinition>,
}

impl AllCrops {
    fn from(assets: &HardcodedCropAssetsThatShouldBeTurnedIntoDynamicResourcesEventually) -> Self {
        let mut definitions = HashMap::new();

        definitions.insert(
            CropId(0),
            CropDefinition {
                id: CropId(0),
                name: String::from("Blue Debug Plant"),
                stages: 4,
                growth_time_per_stage: 5,
                texture_atlas: assets.blue_debug_plant.clone(),
                harvested_sprite: assets.blue_debug_veggie.clone(),
            },
        );
        definitions.insert(
            CropId(1),
            CropDefinition {
                id: CropId(1),
                name: String::from("Red Debug Plant"),
                stages: 4,
                growth_time_per_stage: 1,
                texture_atlas: assets.red_debug_plant.clone(),
                harvested_sprite: assets.red_debug_veggie.clone(),
            },
        );

        Self { definitions }
    }
}

pub struct CropDefinition {
    pub id: CropId,
    pub name: String,
    pub stages: u8,
    pub growth_time_per_stage: u32,
    pub texture_atlas: Handle<TextureAtlas>,
    pub harvested_sprite: Handle<Image>,
}

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub struct CropId(pub u32);
