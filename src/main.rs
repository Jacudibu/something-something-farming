mod game;
mod load;
mod prelude;

use crate::game::GamePlugin;
use crate::load::LoadingPlugin;
use crate::prelude::DebugOverlayPlugin;
use bevy::prelude::*;
use bevy::window::PresentMode;
use bevy_kira_audio::AudioPlugin;
use bevy_screen_diagnostics::{
    ScreenDiagnosticsPlugin, ScreenEntityDiagnosticsPlugin, ScreenFrameDiagnosticsPlugin,
};
use bevy_sprite3d::Sprite3dPlugin;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins
                .set(ImagePlugin::default_nearest())
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "Your ad could be placed here!".to_string(),
                        present_mode: PresentMode::AutoNoVsync,
                        ..default()
                    }),
                    ..default()
                }),
            AudioPlugin,
        ))
        .add_state::<GameState>()
        .add_state::<SoundEffectsSetting>()
        .add_plugins(Sprite3dPlugin)
        .add_plugins(LoadingPlugin)
        .add_plugins(GamePlugin)
        .add_plugins(ScreenDiagnosticsPlugin::default())
        .add_plugins(ScreenFrameDiagnosticsPlugin)
        .add_plugins(ScreenEntityDiagnosticsPlugin)
        .add_plugins(DebugOverlayPlugin)
        .run();
}

#[derive(Clone, Eq, PartialEq, Debug, Hash, Default, States)]
pub enum GameState {
    #[default]
    Loading,
    Playing,
}

#[derive(Clone, Eq, PartialEq, Debug, Hash, Default, States)]
pub enum SoundEffectsSetting {
    #[default]
    On,
    Off,
}
