#[derive(Debug, Copy, Clone)]
pub enum GroundType {
    Grass,
}

impl GroundType {
    pub fn texture_index(&self) -> usize {
        match self {
            GroundType::Grass => 2,
        }
    }
}
