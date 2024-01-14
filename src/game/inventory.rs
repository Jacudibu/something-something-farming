use crate::prelude::item_id::ItemId;
use bevy::prelude::Component;
use bevy::utils::{hashbrown, HashMap};

#[derive(Component, Default)]
pub struct Inventory {
    items: HashMap<ItemId, u32>,
}

impl<'a> IntoIterator for &'a Inventory {
    type Item = (&'a ItemId, &'a u32);
    type IntoIter = hashbrown::hash_map::Iter<'a, ItemId, u32>;

    fn into_iter(self) -> Self::IntoIter {
        self.items.iter()
    }
}

impl Inventory {
    pub fn is_empty(&self) -> bool {
        self.items.is_empty()
    }

    pub fn add_item(&mut self, item_id: &ItemId, amount: u32) {
        if let Some(count) = self.items.get_mut(item_id) {
            *count += amount;
        } else {
            self.items.insert(item_id.clone(), amount);
        }
    }

    pub fn item_count(self, item_id: ItemId) -> u32 {
        if let Some(count) = self.items.get(&item_id) {
            count.clone()
        } else {
            0
        }
    }
}
