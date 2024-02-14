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

const GRID_SIZE: f32 = 1.0;
const SUBGRID_SIZE: f32 = 0.1;
const SUBGRID_COLOR: Color = Color::rgba(0.0, 0.0, 0.0, 0.5);

fn draw_subgrid(mut gizmos: Gizmos, query: Query<&TileCursor>) {
    for cursor in query.iter() {
        let mouse_location = cursor.mouse_pos.clone() + Vec3::new(0.5, 0.0, 0.5);
        let x_mouse_offset = mouse_location.x.fract().abs();
        let z_mouse_offset = mouse_location.z.fract().abs();

        let subgrid_x = (x_mouse_offset / SUBGRID_SIZE).floor() * SUBGRID_SIZE;
        let subgrid_z = (z_mouse_offset / SUBGRID_SIZE).floor() * SUBGRID_SIZE;

        const SUBGRID_RADIUS: i8 = 10;
        const RADIUS: f32 = SUBGRID_RADIUS as f32 * SUBGRID_SIZE;

        let grid_anchor =
            cursor.pos.world_pos(0.0) + Vec3::new(subgrid_x - 0.45, 0.0, subgrid_z - 0.45);

        gizmos.circle(grid_anchor, Vec3::Y, 0.1, Color::BLACK);

        for row in 0..SUBGRID_RADIUS {
            let line_origin = row as f32 * SUBGRID_SIZE + 0.05;
            let line_length = (SUBGRID_RADIUS - row) as f32 * SUBGRID_SIZE;

            // Horizontal
            let pos = grid_anchor + Vec3::new(0.0, 0.0, line_origin);
            gizmos.line(pos, pos + Vec3::new(line_length, 0.0, 0.0), SUBGRID_COLOR);

            // Vertical
            let pos = grid_anchor + Vec3::new(line_origin, 0.0, 0.0);
            gizmos.line(pos, pos + Vec3::new(0.0, 0.0, line_length), SUBGRID_COLOR);
        }
    }
}
