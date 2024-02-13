use std::fmt::{Display, Formatter};

use crate::prelude::AllItems;

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub struct CropId(pub u32);

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub enum ToolId {
    Hoe,
    Pickaxe,
    Scythe,
}

impl Display for ToolId {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ToolId::Hoe => write!(f, "Hoe"),
            ToolId::Pickaxe => write!(f, "Pickaxe"),
            ToolId::Scythe => write!(f, "Scythe"),
        }
    }
}

#[derive(Eq, PartialEq, Hash, Copy, Clone, Debug)]
pub enum ItemId {
    Crop { crop_id: CropId },
    Seed { crop_id: CropId },
    Tool { tool_id: ToolId },
}

impl Display for ItemId {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ItemId::Crop { crop_id } => write!(f, "Crop (ID {})", crop_id.0),
            ItemId::Seed { crop_id } => write!(f, "Seed (ID {})", crop_id.0),
            ItemId::Tool { tool_id } => tool_id.fmt(f),
        }
    }
}

impl ItemId {
    pub fn item_name(&self, all_items: &AllItems) -> String {
        match self {
            ItemId::Crop { crop_id } => all_items.crops[crop_id].name.clone(),
            ItemId::Seed { crop_id } => {
                format!("{} Seed", all_items.crops[crop_id].name.clone())
            }
            ItemId::Tool { tool_id } => tool_id.to_string(),
        }
    }
}
