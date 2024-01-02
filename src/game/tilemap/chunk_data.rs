use bevy::math::IVec2;
use bevy::prelude::Component;

#[derive(Component)]
pub struct ChunkData {
    pub position: IVec2,
}
