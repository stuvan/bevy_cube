use bevy::prelude::*;
use bevy_rapier3d::prelude::*;
use platform_component::PlatformMarker;

mod platform_component;

pub struct PlatformPlugin;

impl Plugin for PlatformPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_platform);
    }
}

fn spawn_platform(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // Platform
    commands.spawn((
        PbrBundle {
            mesh: meshes.add(Cuboid::new(1000.0, 0.2, 10.0)),
            material: materials.add(Color::srgb_u8(255, 255, 255)),
            transform: Transform::from_xyz(495.0, -0.2, 0.0),
            ..default()
        },
        Collider::cuboid(500.0, 0.1, 5.0),
        Friction::coefficient(0.0),
        RigidBody::Fixed,
        PlatformMarker,
    ));
}
