use bevy::app::App;
use bevy::core::Name;
use bevy::math::{EulerRot, Quat};
use bevy::pbr::{CascadeShadowConfigBuilder, DirectionalLight, DirectionalLightBundle};
use bevy::prelude::{default, Commands, OnEnter, Plugin, Transform};

use crate::GameState;

pub struct LightPlugin;
impl Plugin for LightPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Playing), init);
    }
}

fn init(mut commands: Commands) {
    // Simple minded ez light rotations
    //  X    Y
    // -1.0  1 Morning
    // -1.2  0 Noon
    // -1.0 -1 Evening
    commands.spawn((
        Name::new("Directional Light"),
        DirectionalLightBundle {
            directional_light: DirectionalLight {
                shadows_enabled: true,
                ..default()
            },
            // TODO: Figure out some good looking values
            cascade_shadow_config: CascadeShadowConfigBuilder {
                first_cascade_far_bound: 20.0,
                minimum_distance: 1.0,
                ..default()
            }
            .build(),
            transform: Transform::from_rotation(Quat::from_euler(EulerRot::XYZ, 5.7, 0.3, 0.0)),
            ..default()
        },
    ));
}
