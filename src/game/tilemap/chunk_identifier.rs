use crate::prelude::ChunkPos;
use bevy::prelude::Component;

#[derive(Component)]
pub struct ChunkIdentifier {
    pub position: ChunkPos,
}
