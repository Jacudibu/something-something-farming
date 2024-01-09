mod game;
mod prelude;

use crate::game::GamePlugin;
use crate::prelude::DebugOverlayPlugin;
use bevy::prelude::*;
use bevy::window::PresentMode;
use bevy_asset_loader::prelude::*;
use bevy_screen_diagnostics::{
    ScreenDiagnosticsPlugin, ScreenEntityDiagnosticsPlugin, ScreenFrameDiagnosticsPlugin,
};

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(ImagePlugin::default_nearest())
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "Something something farming".to_string(),
                        present_mode: PresentMode::AutoNoVsync,
                        ..default()
                    }),
                    ..default()
                }),
        )
        .add_state::<GameState>()
        .add_loading_state(
            LoadingState::new(GameState::Loading)
                .continue_to_state(GameState::Playing)
                .load_collection::<SpriteAssets>(),
        )
        .insert_resource(CropDefinition {
            id: CropId(0),
            stages: 4,
            growth_time_per_stage: 5,
        })
        .add_plugins(GamePlugin)
        .add_plugins(ScreenDiagnosticsPlugin::default())
        .add_plugins(ScreenFrameDiagnosticsPlugin)
        .add_plugins(ScreenEntityDiagnosticsPlugin)
        .add_plugins(DebugOverlayPlugin)
        .run();
}

#[derive(Clone, Eq, PartialEq, Debug, Hash, Default, States)]
enum GameState {
    #[default]
    Loading,
    Playing,
}

#[derive(AssetCollection, Resource)]
struct SpriteAssets {
    #[asset(texture_atlas(tile_size_x = 16.0, tile_size_y = 16.0, columns = 4, rows = 1))]
    #[asset(path = "sprites/debug_plant.png")]
    plant: Handle<TextureAtlas>,

    #[asset(path = "sprites/tile_cursor.png")]
    cursor: Handle<Image>,
    #[asset(path = "sprites/tilled_tile.png")]
    tilled_tiles: Handle<Image>,
    #[asset(path = "sprites/simple_tiles.png")]
    simple_tiles: Handle<Image>,
}

#[derive(Resource)]
struct CropDefinition {
    id: CropId,
    stages: u8,
    growth_time_per_stage: u32,
}

#[derive(Copy, Clone, Eq, PartialEq)]
struct CropId(u32);
