use bevy::prelude::*;
use bevy_editor_pls::EditorPlugin;
use bevy_rapier3d::prelude::*;
use camera::CameraPlugin;
use platform::PlatformPlugin;
use player::PlayerPlugin;

mod camera;
mod platform;
mod player;

fn main() {
    let mut app = App::new();
    // Default and Development Plugins
    app.add_plugins((DefaultPlugins, EditorPlugin::new()));

    // Physics Simulation using Avain3d
    app.add_plugins(RapierPhysicsPlugin::<NoUserData>::default())
        .add_plugins(RapierDebugRenderPlugin::default());

    // Game Plugins
    app.add_plugins((CameraPlugin, PlayerPlugin, PlatformPlugin));

    app.run();
}
