use bevy::math::{Quat, Vec2, Vec3};
use bevy::prelude::{
    default, in_state, App, Color, Condition, GizmoConfig, Gizmos, GlobalTransform,
    IntoSystemConfigs, Plugin, Query, States, Update, With,
};

use crate::prelude::debug_actions::DebugOverlayState;
use crate::prelude::tile_cursor::TileCursor;
use crate::prelude::TilePos;

pub struct TileGridGizmo;
impl Plugin for TileGridGizmo {
    fn build(&self, app: &mut App) {
        app.add_state::<TileGridRenderingState>();
        app.add_state::<SubGridRenderingState>();
        app.insert_resource(GizmoConfig {
            depth_bias: -0.0001,
            ..default()
        });
        app.add_systems(
            Update,
            draw_grid.run_if(
                in_state(TileGridRenderingState::Visible).or_else(in_state(DebugOverlayState::On)),
            ),
        );
        app.add_systems(
            Update,
            draw_subgrid.run_if(
                in_state(SubGridRenderingState::Visible).or_else(in_state(DebugOverlayState::On)),
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

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
pub enum SubGridRenderingState {
    #[default]
    Hidden,
    Visible,
}

fn draw_grid(mut gizmos: Gizmos, query: Query<&GlobalTransform, With<TilePos>>) {
    for transform in query.iter() {
        gizmos.rect(
            transform.translation(),
            Quat::from_rotation_x(f32::to_radians(90.0)),
            Vec2::ONE,
            Color::rgba(0.0, 0.0, 0.0, 0.8),
        );
    }
}

const SUBGRID_SIZE: f32 = 0.1;
const SUBGRID_COLOR: Color = Color::rgba(0.0, 0.0, 0.0, 0.5);

fn draw_subgrid(mut gizmos: Gizmos, query: Query<&TileCursor>) {
    for cursor in query.iter() {
        // TODO: mouse location should be the middle of the sub-grid cell?
        let mouse_location = cursor.mouse_pos;

        const SUBGRID_RADIUS: u8 = 10;
        const RADIUS: f32 = SUBGRID_RADIUS as f32 * SUBGRID_SIZE;

        let upper_left_edge = mouse_location + Vec3::new(-RADIUS, 0.0, -RADIUS);

        for offset in 0..SUBGRID_RADIUS as i32 {
            let line_length = offset as f32 * SUBGRID_SIZE;
            let empty_length = RADIUS - line_length;

            // Horizontal
            let pos = upper_left_edge + Vec3::new(empty_length, 0.0, line_length);
            gizmos.line(pos, pos + Vec3::new(line_length, 0.0, 0.0), SUBGRID_COLOR);

            // Vertical
            let pos = upper_left_edge + Vec3::new(line_length, 0.0, empty_length);
            gizmos.line(pos, pos + Vec3::new(0.0, 0.0, line_length), SUBGRID_COLOR);
        }
    }
}
