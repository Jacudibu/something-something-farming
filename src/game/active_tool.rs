use std::fmt::{Debug, Display, Formatter};

use bevy::prelude::Resource;

use crate::game::item_id::ItemId;

#[derive(Resource, Copy, Clone, Debug)]
pub enum ActiveTool {
    None,
    Item(ItemId),
    Wall,
}

impl Display for ActiveTool {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ActiveTool::Item(item) => std::fmt::Display::fmt(&item, f),
            ActiveTool::Wall => write!(f, "Wall"),
            ActiveTool::None => write!(f, "None"),
        }
    }
}

impl Default for ActiveTool {
    fn default() -> Self {
        ActiveTool::None
    }
}
