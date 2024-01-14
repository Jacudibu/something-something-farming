use crate::load::AllCrops;
use std::fmt::{Display, Formatter};

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
    Tool { tool_id: ToolId },
}

impl ItemId {
    pub fn item_name(&self, all_crops: &AllCrops) -> String {
        match self {
            ItemId::Crop { crop_id } => all_crops.definitions[crop_id].name.clone(),
            ItemId::Tool { tool_id } => tool_id.to_string(),
        }
    }
}
