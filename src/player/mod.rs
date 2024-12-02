use bevy::prelude::*;
use bevy_rapier3d::prelude::*;
use player_component::Player;

pub mod player_component;

const MOVEMENT_SPEED: f32 = 10.0;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems(Startup, setup_player);
        app.add_systems(FixedUpdate, player_movement);
    }
}

// Spawn Player
fn setup_player(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // Player
    commands.spawn((
        PbrBundle {
            mesh: meshes.add(Cuboid::new(1.0, 1.0, 1.0)),
            material: materials.add(Color::srgb_u8(255, 0, 0)),
            transform: Transform::from_xyz(0.0, 0.5, 0.0),
            ..default()
        },
        Player,
        RigidBody::Dynamic,
        Velocity::default(),
        Friction::coefficient(0.0),
        Collider::cuboid(0.5, 0.5, 0.5),
    ));
}

// Movement For Player
fn player_movement(
    mut query: Query<(&Player, &mut Velocity)>,
    time: Res<Time>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
) {
    for (_, mut linear_velocity) in &mut query {
        linear_velocity.linvel.x += MOVEMENT_SPEED * time.delta_seconds();

        let left = keyboard_input.any_pressed([KeyCode::KeyA, KeyCode::ArrowLeft]);
        let right = keyboard_input.any_pressed([KeyCode::KeyD, KeyCode::ArrowRight]);

        let horizontal = right as i8 - left as i8;
        if horizontal != 0 {
            linear_velocity.linvel.z += horizontal as f32 * MOVEMENT_SPEED * time.delta_seconds();
        } else {
            if linear_velocity.linvel.z < 0.0 {
                linear_velocity.linvel.z += MOVEMENT_SPEED * time.delta_seconds();
            } else if linear_velocity.linvel.z > 0.0 {
                linear_velocity.linvel.z -= MOVEMENT_SPEED * time.delta_seconds();
            }
        }
    }
}
