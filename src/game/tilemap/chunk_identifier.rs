use bevy::math::IVec2;
use bevy::prelude::Component;

#[derive(Component)]
pub struct ChunkIdentifier {
    pub position: IVec2,
}
