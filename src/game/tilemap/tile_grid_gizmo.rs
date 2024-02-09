use bevy::math::{Quat, Vec2};
use bevy::prelude::{
    default, in_state, App, Color, Condition, GizmoConfig, Gizmos, GlobalTransform,
    IntoSystemConfigs, Plugin, Query, States, Update, With,
};

use crate::game::debug_overlay::DebugOverlayState;
use crate::prelude::TilePos;

pub struct TileGridGizmo;
impl Plugin for TileGridGizmo {
    fn build(&self, app: &mut App) {
        app.add_state::<TileGridRenderingState>();
        app.insert_resource(GizmoConfig {
            depth_bias: -0.0001,
            ..default()
        });
        app.add_systems(
            Update,
            draw.run_if(
                in_state(TileGridRenderingState::Visible).or_else(in_state(DebugOverlayState::On)),
            ),
        );
    }
}

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
pub enum TileGridRenderingState {
    #[default]
    Hidden,
    Visible,
}

fn draw(mut gizmos: Gizmos, query: Query<&GlobalTransform, With<TilePos>>) {
    for transform in query.iter() {
        gizmos.rect(
            transform.translation(),
            Quat::from_rotation_x(f32::to_radians(90.0)),
            Vec2::ONE,
            Color::rgba(0.0, 0.0, 0.0, 0.8),
        );
    }
}
