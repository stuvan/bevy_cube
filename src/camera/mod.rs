use bevy::prelude::*;
use camera_component::CameraMarker;

use crate::player::player_component::Player;

pub mod camera_component;

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems(Startup, setup_environment);
        app.add_systems(Update, camera_follow_player);
    }
}

fn setup_environment(mut commands: Commands) {
    commands.spawn(DirectionalLightBundle {
        directional_light: DirectionalLight {
            illuminance: light_consts::lux::FULL_DAYLIGHT,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform {
            translation: Vec3::new(-2.0, -0.3, 0.0),
            rotation: Quat::from_axis_angle(Vec3::new(-1.0, 0.0, 1.0), 2.0),
            ..default()
        },
        ..default()
    });

    // camera
    commands.spawn((
        Camera3dBundle {
            transform: Transform::from_xyz(-6., 2., 0.)
                .looking_at(Vec3::new(0.0, 2.0, 0.0), Vec3::Y),
            ..default()
        },
        CameraMarker,
    ));
}

fn camera_follow_player(
    player_query: Query<(&Transform, &Player)>,
    mut camera_query: Query<(&mut Transform, &CameraMarker), Without<Player>>,
) {
    let (player, _) = player_query.single();
    let (mut camera, _) = camera_query.single_mut();

    let desired_position = Vec3::new(
        player.translation.x - 6.0,
        player.translation.y + 2.0,
        player.translation.z,
    );

    camera.translation = desired_position
}
