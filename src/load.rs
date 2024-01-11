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
}

#[derive(Resource, AssetCollection)]
struct HardcodedCropAssetsThatShouldBeTurnedIntoDynamicResourcesEventually {
    #[asset(texture_atlas(tile_size_x = 16.0, tile_size_y = 16.0, columns = 4, rows = 1))]
    #[asset(path = "sprites/debug_plant.png")]
    pub plant: Handle<TextureAtlas>,
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
                texture_atlas: assets.plant.clone(),
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
}

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct CropId(pub u32);
