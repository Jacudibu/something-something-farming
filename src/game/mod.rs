use bevy::app::{App, First, Plugin};
use bevy::math::Vec2;
use bevy::prelude::{Camera, CursorMoved, EventReader, GlobalTransform, Query, ResMut, Resource};
use leafwing_input_manager::plugin::InputManagerPlugin;

use player::PlayerAction;

use crate::game::camera::CameraPlugin;
use crate::game::drops::ItemPickupPlugin;
use crate::game::interaction::InteractionPlugin;
use crate::game::tile_updater::TileUpdaterPlugin;
use crate::game::tilemap::GameMapPlugin;
use crate::game::ui::UiPlugin;
use crate::game::world_data::WorldDataPlugin;
use crate::prelude::simulation_time::SimulationTimePlugin;
use crate::prelude::PlayerPlugin;

pub mod active_tool;
pub mod camera;
pub mod debug_overlay;
mod drops;
pub mod interaction;
pub(crate) mod inventory;
pub mod item_id;
pub mod map_pos;
pub mod player;
pub mod simulation_time;
pub mod tile_updater;
pub mod tilemap;
pub mod ui;
pub mod world_data;

pub const TILE_SIZE: f32 = 16.0;
pub const CHUNK_SIZE: usize = 32;

pub const SPRITE_PIXELS_PER_METER: f32 = 16.0;
pub const SPRITE_DEFAULT_PIVOT: Option<Vec2> = Some(Vec2::new(0.5, 0.0));

pub struct GamePlugin;
impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<CursorPos>()
            .add_plugins(InputManagerPlugin::<PlayerAction>::default())
            .add_plugins(SimulationTimePlugin)
            .add_plugins(ItemPickupPlugin)
            .add_plugins(PlayerPlugin)
            .add_plugins(WorldDataPlugin)
            .add_plugins(GameMapPlugin)
            .add_plugins(CameraPlugin)
            .add_plugins(InteractionPlugin)
            .add_plugins(UiPlugin)
            .add_plugins(TileUpdaterPlugin)
            .add_systems(First, update_cursor_pos);
    }
}

#[derive(Resource)]
pub struct CursorPos {
    pub screen: Vec2,
    pub world: Vec2,
}
impl Default for CursorPos {
    fn default() -> Self {
        CursorPos {
            screen: Vec2::new(-10000.0, -10000.0),
            world: Vec2::new(-10000.0, -10000.0),
        }
    }
}

pub fn update_cursor_pos(
    camera_q: Query<(&GlobalTransform, &Camera)>,
    mut cursor_moved_events: EventReader<CursorMoved>,
    mut cursor_pos: ResMut<CursorPos>,
) {
    // TODO: If we are using the gamepad, cursorpos should be playerPos + stick * range
    for cursor_moved in cursor_moved_events.read() {
        cursor_pos.screen = cursor_moved.position;
        for (cam_t, cam) in camera_q.iter() {
            if let Some(pos) = cam.viewport_to_world_2d(cam_t, cursor_moved.position) {
                cursor_pos.world = pos;
            }
        }
    }
}
