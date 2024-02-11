use bevy::core::Name;
use bevy::log::info;
use bevy::math::{Quat, Vec3};
use bevy::prelude::{
    default, BuildChildren, Commands, Component, Entity, PbrBundle, SpatialBundle, Transform,
};
use bevy::utils::info;

use crate::prelude::{CardinalDirection, DebugMaterials, DebugMeshes};

const TILE_EDGE: f32 = 0.5;
const WALL_WIDTH: f32 = 0.1;

pub fn build_wall(
    commands: &mut Commands,
    tile: Entity,
    tile_edge: CardinalDirection,
    debug_meshes: &DebugMeshes,
    debug_materials: &DebugMaterials,
) -> Entity {
    info!("placing {:?}", tile_edge);
    return commands
        .spawn((
            Name::new("Wall"),
            PbrBundle {
                mesh: debug_meshes.wall.clone(),
                material: debug_materials.wall.clone(),
                transform: Transform {
                    translation: tile_edge_to_position(tile_edge),
                    rotation: tile_edge_to_rotation(tile_edge),
                    ..default()
                },
                ..default()
            },
        ))
        .set_parent(tile)
        .id();
}

fn tile_edge_to_position(cardinal_direction: CardinalDirection) -> Vec3 {
    match cardinal_direction {
        CardinalDirection::North => Vec3::new(0.0, 1.0, -TILE_EDGE + WALL_WIDTH * 0.5),
        CardinalDirection::East => Vec3::new(TILE_EDGE - WALL_WIDTH * 0.5, 1.0, 0.0),
        CardinalDirection::South => Vec3::new(0.0, 1.0, TILE_EDGE - WALL_WIDTH * 0.5),
        CardinalDirection::West => Vec3::new(-TILE_EDGE + WALL_WIDTH * 0.5, 1.0, 0.0),
    }
}

fn tile_edge_to_rotation(cardinal_direction: CardinalDirection) -> Quat {
    match cardinal_direction {
        CardinalDirection::North => Quat::from_rotation_y(std::f32::consts::PI),
        CardinalDirection::East => Quat::from_rotation_y(std::f32::consts::FRAC_PI_2),
        CardinalDirection::South => Quat::from_rotation_y(0.0),
        CardinalDirection::West => Quat::from_rotation_y(-std::f32::consts::FRAC_PI_2),
    }
}

// TODO: Re-Test if this segmented approach is more performant once we check if a wall has already been placed & once we combine meshes.
#[derive(Component)]
#[allow(dead_code)]
pub struct WallParent {
    outer: Entity,
    inner: Entity,
    top: Option<Entity>,
    left: Option<Entity>,
    right: Option<Entity>,
}

#[allow(dead_code)]
pub fn build_segmented_wall(
    commands: &mut Commands,
    tile: Entity,
    tile_edge: CardinalDirection,
    debug_meshes: &DebugMeshes,
    debug_materials: &DebugMaterials,
) -> Entity {
    let outer = commands
        .spawn((
            Name::new("Outer"),
            (PbrBundle {
                transform: Transform::from_translation(Vec3::new(0.0, 0.0, TILE_EDGE)),
                mesh: debug_meshes.wall_segment_front.clone(),
                material: debug_materials.wall.clone(),
                ..default()
            }),
        ))
        .id();

    let inner = commands
        .spawn((
            Name::new("Inner"),
            (PbrBundle {
                transform: Transform {
                    translation: Vec3::new(0.0, 0.0, TILE_EDGE - WALL_WIDTH),
                    rotation: Quat::from_rotation_y(std::f32::consts::PI),
                    ..default()
                },
                mesh: debug_meshes.wall_segment_front.clone(),
                material: debug_materials.wall.clone(),
                ..default()
            }),
        ))
        .id();

    let top = commands
        .spawn((
            Name::new("Top"),
            (PbrBundle {
                transform: Transform {
                    translation: Vec3::new(0.0, 1.0, TILE_EDGE - WALL_WIDTH + WALL_WIDTH * 0.5),
                    rotation: Quat::from_rotation_x(-std::f32::consts::FRAC_PI_2),
                    ..default()
                },
                mesh: debug_meshes.wall_segment_top.clone(),
                material: debug_materials.wall.clone(),
                ..default()
            }),
        ))
        .id();

    let parent = commands
        .spawn((
            Name::new("Wall Parent"),
            SpatialBundle {
                transform: Transform {
                    translation: Vec3::new(0.0, 1.0, 0.0),
                    rotation: tile_edge_to_rotation(tile_edge),
                    ..default()
                },
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
