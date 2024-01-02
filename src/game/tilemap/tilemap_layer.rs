use bevy::prelude::Component;
use std::fmt::{Display, Formatter};

#[derive(Component)]
pub struct GroundLayer;

#[derive(Component, PartialOrd, PartialEq)]
pub enum TilemapLayer {
    Ground, // The actual natural ground
    Floor,  // Walkable stuff placed on top of the ground
}

impl Into<f32> for TilemapLayer {
    fn into(self) -> f32 {
        match self {
            TilemapLayer::Ground => 0.0,
            TilemapLayer::Floor => 1.0,
        }
    }
}

impl Display for TilemapLayer {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            TilemapLayer::Ground => write!(f, "Ground"),
            TilemapLayer::Floor => write!(f, "Floor"),
        }
    }
}
