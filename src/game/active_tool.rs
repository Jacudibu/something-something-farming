use bevy::prelude::Resource;
use std::fmt::{Display, Formatter};

#[derive(Resource, Copy, Clone, Debug)]
pub enum ActiveTool {
    Hoe,
    Pickaxe,
    Seed,
}

impl Display for ActiveTool {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ActiveTool::Hoe => write!(f, "Hoe"),
            ActiveTool::Pickaxe => write!(f, "Pickaxe"),
            ActiveTool::Seed => write!(f, "Seed"),
        }
    }
}
