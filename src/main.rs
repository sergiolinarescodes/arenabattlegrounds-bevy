mod camera_plugin;
mod handle_object;
mod player_plugin;
use bevy::prelude::*;

use camera_plugin::CameraPlugin;
use handle_object::HandleObjectPlugin;
use player_plugin::PlayerPlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(HandleObjectPlugin)
        .add_plugins(PlayerPlugin)
        .add_plugins(CameraPlugin)
        .run();
}
