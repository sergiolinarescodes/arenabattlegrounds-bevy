use bevy::prelude::*;

#[derive(Bundle)]
pub struct ObjectBundle {
    pub model: PbrBundle,
}

pub struct HandleObjectPlugin;

impl Plugin for HandleObjectPlugin {
    fn build(&self, app: &mut App) {}
}
