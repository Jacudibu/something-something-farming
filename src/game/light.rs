use bevy::app::App;
use bevy::core::Name;
use bevy::log::info;
use bevy::math::{EulerRot, Quat};
use bevy::pbr::{CascadeShadowConfigBuilder, DirectionalLight, DirectionalLightBundle};
use bevy::prelude::{default, Commands, OnEnter, Plugin, Query, Res, Transform, Update, With};

use crate::prelude::SimulationDate;
use crate::GameState;

pub struct LightPlugin;
impl Plugin for LightPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Playing), init)
            .add_systems(Update, update_sun_rotation);
    }
}

fn init(mut commands: Commands) {
    // Simple minded ez light rotations
    //  X    Y
    // -1.0  1 Morning
    // -1.2  0 Noon
    // -1.0 -1 Evening
    commands.spawn((
        Name::new("Sun"),
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

fn update_sun_rotation(
    mut query: Query<&mut Transform, With<DirectionalLight>>,
    simulation_date: Res<SimulationDate>,
) {
    let Ok(mut sun) = query.get_single_mut() else {
        info!("There should always be exactly one sun, right? Probably time to add a marker component!");
        return;
    };

    // TODO: Change color depending on t.

    let t =
        1.0 - (simulation_date.hour as f32 / 24.0 + simulation_date.minute as f32 / (60.0 * 24.0));
    let radian = std::f32::consts::PI * 2.0 * t - std::f32::consts::PI;

    sun.rotation = Quat::from_euler(EulerRot::XYZ, -1.1, radian, 0.0);
}
