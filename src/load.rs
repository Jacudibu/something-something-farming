use bevy::prelude::*;
use bevy::utils::HashMap;
use bevy_asset_loader::prelude::*;
use bevy_kira_audio::AudioSource;

use data::prelude::{AllItems, CropDefinition, CropId, PropDefinition, PropId};

use crate::GameState;

pub struct LoadingPlugin;

impl Plugin for LoadingPlugin {
    fn build(&self, app: &mut App) {
        app.add_loading_state(
            LoadingState::new(GameState::Loading)
                .continue_to_state(GameState::Playing)
                .load_collection::<SpriteAssets>()
                .load_collection::<DebugSounds>()
                .load_collection::<DebugTexturesForMaterials>()
                .load_collection::<HardcodedCropAssetsThatShouldBeTurnedIntoDynamicResourcesEventually>()
                .init_resource::<DebugMaterials>()
                .init_resource::<DebugMeshes>(),
        )
            .add_systems(OnExit(GameState::Loading), process_data);
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
struct DebugTexturesForMaterials {
    #[asset(path = "textures/ground/grass.png")]
    pub grass: Handle<Image>,
    #[asset(path = "textures/ground/tilled.png")]
    pub tilled: Handle<Image>,
}

#[derive(Resource, AssetCollection)]
pub struct DebugMeshes {
    pub tile: Handle<Mesh>,
    pub wall: Handle<Mesh>,
    pub torch: Handle<Mesh>,
    pub wall_segment_front: Handle<Mesh>,
    pub wall_segment_top: Handle<Mesh>,
    pub wall_segment_side: Handle<Mesh>,
}

impl FromWorld for DebugMeshes {
    fn from_world(world: &mut World) -> Self {
        let cell = world.cell();
        let mut mesh_assets = cell
            .get_resource_mut::<Assets<Mesh>>()
            .expect("Failed to get Assets<Mesh>");

        DebugMeshes {
            tile: mesh_assets.add(Rectangle::new(1.0, 1.0).into()),
            wall: mesh_assets.add(Cuboid::new(1.0, 2.0, 0.1).into()),
            torch: mesh_assets.add(Cuboid::new(0.1, 0.3, 0.1).into()),
            wall_segment_front: mesh_assets.add(Rectangle::new(1.0, 2.0).into()),
            wall_segment_top: mesh_assets.add(Rectangle::new(1.0, 0.1).into()),
            wall_segment_side: mesh_assets.add(Rectangle::new(0.1, 2.0).into()),
        }
    }
}

#[derive(Resource, AssetCollection)]
pub struct DebugMaterials {
    pub grass: Handle<StandardMaterial>,
    pub tilled: Handle<StandardMaterial>,
    pub wall: Handle<StandardMaterial>,
    pub wall_hidden: Handle<StandardMaterial>,
    pub preview_ghost: Handle<StandardMaterial>,
}

impl FromWorld for DebugMaterials {
    fn from_world(world: &mut World) -> Self {
        let cell = world.cell();
        let debug_textures = cell
            .get_resource::<DebugTexturesForMaterials>()
            .expect("Failed to get DebugTexturesForMaterials");

        let mut standard_materials = cell
            .get_resource_mut::<Assets<StandardMaterial>>()
            .expect("Failed to get Assets<StandardMaterial>");

        DebugMaterials {
            grass: standard_materials.add(StandardMaterial {
                base_color_texture: Some(debug_textures.grass.clone()),
                reflectance: 0.0,
                ..default()
            }),
            tilled: standard_materials.add(StandardMaterial {
                base_color_texture: Some(debug_textures.tilled.clone()),
                reflectance: 0.0,
                ..default()
            }),
            wall: standard_materials.add(StandardMaterial {
                base_color: Color::rgb(0.8, 0.8, 0.8),
                reflectance: 0.3,
                perceptual_roughness: 0.7,
                ..default()
            }),
            wall_hidden: standard_materials.add(StandardMaterial {
                base_color: Color::rgba(0.0, 0.0, 0.0, 0.0),
                alpha_mode: AlphaMode::Multiply,
                ..default()
            }),
            preview_ghost: standard_materials.add(StandardMaterial {
                base_color: Color::rgba(0.0, 0.4, 0.0, 0.4),
                alpha_mode: AlphaMode::Premultiplied,
                ..default()
            }),
        }
    }
}

#[derive(Resource, AssetCollection)]
pub struct DebugSounds {
    #[asset(path = "sounds/plink.ogg")]
    pub plink: Handle<AudioSource>,
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

fn process_data(world: &mut World) {
    let assets = world
        .get_resource::<HardcodedCropAssetsThatShouldBeTurnedIntoDynamicResourcesEventually>()
        .expect("Hardcoded assets should be loaded! :(");

    let meshes = world
        .get_resource::<DebugMeshes>()
        .expect("Failed to get DebugMeshes");
    let materials = world
        .get_resource::<DebugMaterials>()
        .expect("Failed to get DebugMaterials");

    let crops = AllItems {
        crops: parse_crops(&assets),
        props: parse_props(meshes, materials),
    };

    world.insert_resource(crops);
}

fn parse_crops(
    assets: &HardcodedCropAssetsThatShouldBeTurnedIntoDynamicResourcesEventually,
) -> HashMap<CropId, CropDefinition> {
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

    definitions
}

fn parse_props(
    meshes: &DebugMeshes,
    materials: &DebugMaterials,
) -> HashMap<PropId, PropDefinition> {
    let mut definitions = HashMap::new();

    definitions.insert(
        PropId(0),
        PropDefinition {
            id: PropId(0),
            name: String::from("Torch"),
            mesh: meshes.torch.clone(),
            material: materials.wall.clone(),
        },
    );

    definitions
}
