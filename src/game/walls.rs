use bevy::core::Name;
use bevy::math::Vec3;
use bevy::prelude::{
    default, BuildChildren, Commands, Component, Entity, PbrBundle, SpatialBundle, Transform,
};

use crate::prelude::{DebugMaterials, DebugMeshes};

const TILE_EDGE: f32 = 0.5;
const WALL_WIDTH: f32 = 0.1;

#[derive(Component)]
pub struct WallParent {
    outer: Entity,
    inner: Entity,
    top: Option<Entity>,
    left: Option<Entity>,
    right: Option<Entity>,
}

pub fn build_wall(
    commands: &mut Commands,
    tile: Entity,
    debug_meshes: &DebugMeshes,
    debug_materials: &DebugMaterials,
) -> Entity {
    let outer = commands
        .spawn((
            Name::new("Outer"),
            (PbrBundle {
                transform: Transform::from_translation(Vec3::new(0.0, 0.0, TILE_EDGE)),
                mesh: debug_meshes.wall.clone(),
                material: debug_materials.wall.clone(),
                ..default()
            }),
        ))
        .id();

    let inner = commands
        .spawn((
            Name::new("Inner"),
            (PbrBundle {
                transform: Transform::from_translation(Vec3::new(0.0, 0.0, TILE_EDGE - WALL_WIDTH)),
                mesh: debug_meshes.wall.clone(),
                material: debug_materials.wall.clone(),
                ..default()
            }),
        ))
        .id();

    let top = commands
        .spawn((
            Name::new("Top"),
            (PbrBundle {
                transform: Transform::from_translation(Vec3::new(
                    0.0,
                    2.0,
                    TILE_EDGE - WALL_WIDTH + WALL_WIDTH * 0.5,
                )),
                mesh: debug_meshes.wall_top.clone(),
                material: debug_materials.wall.clone(),
                ..default()
            }),
        ))
        .id();

    let parent = commands
        .spawn((
            Name::new("Wall Parent"),
            SpatialBundle {
                transform: Transform::from_xyz(0.0, 1.0, 0.0),
                ..default()
            },
            WallParent {
                outer,
                inner,
                top: Some(top),
                left: None,
                right: None,
            },
        ))
        .set_parent(tile)
        .add_child(outer)
        .add_child(inner)
        .add_child(top)
        .id();

    parent
}
