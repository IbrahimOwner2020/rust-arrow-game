use bevy::prelude::*;

const BOUNDS: Vec2 = Vec2::new(1200.0, 640.0);

fn main() {
    println!("App has started");

    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(FixedUpdate, player_movement_system)
        .run();
}

#[derive(Component)]
struct Player {
    movement_speed: f32,
    rotation_speed: f32,
}

fn setup(mut commands: Commands, server: Res<AssetServer>) {
    let ship_handle: Handle<Image> = server.load("ship_C.png");

    commands.spawn(Camera2dBundle::default());

    commands.spawn((
        SpriteBundle {
            texture: ship_handle,
            ..Default::default()
        },
        Player {
            movement_speed: 200.0,
            rotation_speed: f32::to_radians(360.0),
        },
    ));
}

fn player_movement_system(
    time: Res<Time>,
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<(&Player, &mut Transform)>,
) {
    let (ship, mut transform) = query.single_mut();
    let mut rotation_factor = 0.0;
    let mut movement_factor = 0.0;

    if keyboard_input.pressed(KeyCode::Left) {
        rotation_factor += 1.0;
    }

    if keyboard_input.pressed(KeyCode::Right) {
        rotation_factor -= 1.0;
    }

    if keyboard_input.pressed(KeyCode::Up) {
        movement_factor += 1.0;
    }

    if keyboard_input.pressed(KeyCode::Down) {
        movement_factor -= 1.0;
    }

    // updates ships rotation
    transform.rotate_z(rotation_factor * ship.rotation_speed * time.delta_seconds());

    let movement_direction = transform.rotation * Vec3::Y;

    let movement_distance = movement_factor * ship.movement_speed * time.delta_seconds();

    let tranlation_delta = movement_direction * movement_distance;

    transform.translation += tranlation_delta;

    let extents = Vec3::from((BOUNDS / 2.0, 0.0));
    transform.translation = transform.translation.min(extents).max(-extents);
}
