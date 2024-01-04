use crate::prelude::ground_type::GroundType;

#[derive(Copy, Clone)]
pub struct TileData {
    pub ground_type: GroundType,
    pub is_tilled: bool,
}

impl Default for TileData {
    fn default() -> Self {
        TileData {
            ground_type: GroundType::Grass,
            is_tilled: false,
        }
    }
}
