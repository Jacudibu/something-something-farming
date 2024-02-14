use bevy::math::{Quat, Vec2, Vec3};
use bevy::prelude::{
    default, in_state, App, Color, Condition, GizmoConfig, Gizmos, GlobalTransform,
    IntoSystemConfigs, Plugin, Query, Res, States, Update, With,
};

use crate::prelude::debug_actions::DebugOverlayState;
use crate::prelude::tile_cursor::MouseCursorOnTile;
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
const SUBGRID_RADIUS: i8 = 15;
const SUB_CELLS_PER_TILE: i8 = (1.0 / SUBGRID_SIZE) as i8;

const SUBGRID_MAX_ALPHA: f32 = 0.8;
const SUBGRID_MIN_ALPHA: f32 = 0.0;
const SUBGRID_COLOR_MIN: Color = Color::rgba(0.0, 0.0, 0.0, SUBGRID_MIN_ALPHA);

fn get_start_color(row: i8) -> Color {
    let t = row.abs() as f32 / SUBGRID_RADIUS as f32;
    let alpha = SUBGRID_MAX_ALPHA + (SUBGRID_MIN_ALPHA - SUBGRID_MAX_ALPHA) * t;
    Color::rgba(0.0, 0.0, 0.0, alpha)
}

fn draw_subgrid(mut gizmos: Gizmos, mouse_cursor: Option<Res<MouseCursorOnTile>>) {
    let Some(cursor) = mouse_cursor else {
        return;
    };

    let x_mouse_offset = {
        let x = (cursor.mouse_pos.x + 0.5).fract();
        if x > 0.0 {
            x
        } else {
            x + 1.0
        }
    };
    let z_mouse_offset = {
        let z = (cursor.mouse_pos.z + 0.5).fract();
        if z > 0.0 {
            z
        } else {
            z + 1.0
        }
    };

    let sub_x = (x_mouse_offset / SUBGRID_SIZE).floor();
    let sub_z = (z_mouse_offset / SUBGRID_SIZE).floor();

    let subgrid_x = sub_x * SUBGRID_SIZE;
    let subgrid_z = sub_z * SUBGRID_SIZE;

    let x_subgrid_offset = x_mouse_offset - subgrid_x;
    let z_subgrid_offset = z_mouse_offset - subgrid_z;

    let grid_anchor =
        cursor.tile_pos.world_pos(0.0) + Vec3::new(subgrid_x - 0.50, 0.0, subgrid_z - 0.50);

    for row in -SUBGRID_RADIUS..SUBGRID_RADIUS {
        let line_origin = row as f32 * SUBGRID_SIZE;
        let line_length = (SUBGRID_RADIUS - row.abs()) as f32 * SUBGRID_SIZE;
        let start_color = get_start_color(row);

        // Horizontal
        if (row + sub_z as i8) % SUB_CELLS_PER_TILE != 0 {
            let pos = grid_anchor + Vec3::new(x_subgrid_offset, 0.0, line_origin);
            gizmos.line_gradient(
                pos,
                pos + Vec3::new(line_length, 0.0, 0.0),
                start_color,
                SUBGRID_COLOR_MIN,
            );

            gizmos.line_gradient(
                pos,
                pos + Vec3::new(-line_length, 0.0, 0.0),
                start_color,
                SUBGRID_COLOR_MIN,
            );
        }

        // Vertical
        if (row + sub_x as i8) % SUB_CELLS_PER_TILE != 0 {
            let pos = grid_anchor + Vec3::new(line_origin, 0.0, z_subgrid_offset);
            gizmos.line_gradient(
                pos,
                pos + Vec3::new(0.0, 0.0, line_length),
                start_color,
                SUBGRID_COLOR_MIN,
            );

            gizmos.line_gradient(
                pos,
                pos + Vec3::new(0.0, 0.0, -line_length),
                start_color,
                SUBGRID_COLOR_MIN,
            );
        }
    }
}
