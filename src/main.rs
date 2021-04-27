use bevy::prelude::*;

fn main() {
    //runtime
    App::build()
        .add_plugins(DefaultPlugins)
        // work only once
        .add_startup_system(setup.system())
        .add_startup_stage("game_setup", SystemStage::single(spawn_player.system()))
        .add_system(player_movement.system())
        .run();
}

//Creating Marker Component for Player
struct Player;

struct Player2 {
    velocity: f32,
    direction: Vec2,
}

// impl Default for Player2 {
//     fn default() -> Self {
//         Self {
//             direction: Vec2::new(0.0, 1.0).normalize(),
//             velocity: 200.0,
//         }
//     }
// }

// enum Direction {
//     Left,
//     Up,
//     Right,
//     Down,
// }
//
// impl Direction {
//     fn opposite(self) -> Self {
//         match self {
//             Self::Left => Self::Right,
//             Self::Right => Self::Left,
//             Self::Up => Self::Down,
//             Self::Down => Self::Up,
//         }
//     }
// }

//Creating material component for Player
struct Material(Handle<ColorMaterial>);

struct Velocity(f32);

struct Name(String);

//creating initial setup, with Commands as parameter, commands used as spawner
fn setup(mut commands: Commands, mut material: ResMut<Assets<ColorMaterial>>) {
    // insert (spawn) 2D camera, since this is 2D games we use orthographic camera
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    // spawn the material for player
    commands.insert_resource(Material(material.add(Color::rgb(0.7, 0.7, 0.7).into())));
}

fn spawn_player(mut commands: Commands, materials: Res<Material>) {
    commands
        // here we maake Player as entity
        .spawn_bundle(SpriteBundle {
            material: materials.0.clone(),
            sprite: Sprite::new(Vec2::new(10.0, 10.0)),
            ..Default::default()
        })
        .insert(Player)
        .insert(Velocity(100.0))
        .insert(Name("Bear".to_string()));

    // commands
    //     // here we maake Player2 as entity
    //     .spawn_bundle(SpriteBundle {
    //         material: materials.0.clone(),
    //         sprite: Sprite::new(Vec2::new(20.0, 20.0)),
    //         ..Default::default()
    //     })
    //     .insert(Player2);
}

// Accessing player position from it's transform (included in spritebundle)
fn player_movement(
    // Taking transform component from player entity
    mut movement_data: Query<(&mut Transform, &mut Velocity), With<Player>>,
    time: Res<Time>,
    //mut positions2: Query<&mut Transform, With<Player2>>,
    input: Res<Input<KeyCode>>,
) {
    let delta_time = time.delta_seconds();
    for (mut transform, mut velocity) in movement_data.iter_mut() {
        if input.pressed(KeyCode::Left) {
            transform.translation.x -= delta_time * velocity.0;
        }
        if input.pressed(KeyCode::Right) {
            transform.translation.x += delta_time * velocity.0;
        }
        if input.pressed(KeyCode::Down) {
            transform.translation.y -= delta_time * velocity.0;
        }
        if input.pressed(KeyCode::Up) {
            transform.translation.y += delta_time * velocity.0;
        }
        //speed up
        if input.pressed(KeyCode::W) {
            velocity.0 += 10.0;
        }
        //speed down
        if input.pressed(KeyCode::S) {
            velocity.0 -= 10.0;
        }
        // reset
        if input.pressed(KeyCode::R) {
            transform.translation = Vec3::ZERO;
            velocity.0 = 100.0;
        }
    }

    // for mut transform2 in positions2.iter_mut() {
    //     if input.pressed(KeyCode::A) {
    //         transform2.translation.x -= 2.;
    //     }
    //     if input.pressed(KeyCode::D) {
    //         transform2.translation.x += 2.;
    //     }
    //     if input.pressed(KeyCode::S) {
    //         transform2.translation.y -= 2.;
    //     }
    //     if input.pressed(KeyCode::W) {
    //         transform2.translation.y += 2.;
    //     }
    // }
}
