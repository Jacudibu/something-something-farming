use crate::game::item_id::ItemId;
use bevy::prelude::Resource;
use std::fmt::{Display, Formatter};

#[derive(Resource, Copy, Clone, Debug)]
pub enum ActiveTool {
    Hoe,
    Pickaxe,
    Scythe,
    Item { id: ItemId },
}

impl Display for ActiveTool {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ActiveTool::Hoe => write!(f, "Hoe"),
            ActiveTool::Pickaxe => write!(f, "Pickaxe"),
            ActiveTool::Scythe => write!(f, "Scythe"),
            ActiveTool::Item { id } => match id {
                ItemId::Crop { crop_id } => write!(f, "Crop (ID {})", crop_id.0),
                ItemId::Tool { tool_id } => tool_id.fmt(f),
            },
        }
    }
}
