use bevy::prelude::Resource;

#[derive(Resource)]
pub enum ActiveTool {
    Hoe,
    Pickaxe,
}
