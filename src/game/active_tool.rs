use crate::game::item_id::ItemId;
use bevy::prelude::Resource;
use std::fmt::{Debug, Display, Formatter};

#[derive(Resource, Copy, Clone, Debug)]
pub struct ActiveTool {
    pub item: Option<ItemId>,
}

impl Display for ActiveTool {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        if let Some(item) = self.item {
            std::fmt::Display::fmt(&item, f)
        } else {
            write!(f, "None")
        }
    }
}

impl Default for ActiveTool {
    fn default() -> Self {
        Self { item: None }
    }
}
