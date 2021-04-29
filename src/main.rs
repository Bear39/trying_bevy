use bevy::prelude::*;
use bevy_rapier2d::{
    physics::RapierPhysicsPlugin,
    rapier::{
        dynamics::{RigidBodyBuilder, RigidBodySet},
        geometry::{ColliderBuilder, ColliderSet},
    },
    render::RapierRenderPlugin,
};

fn main() {
    //runtime
    App::build()
        .insert_resource(WindowDescriptor {
            title: "UlerKampret".to_string(),
            resizable: false,
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .add_plugin(RapierPhysicsPlugin)
        //for showing rigid body
        //.add_plugin(RapierRenderPlugin)
        // work only once
        .add_startup_system(setup.system())
        .add_startup_stage("game_setup", SystemStage::single(spawn_players.system()))
        .add_system(player_movement.system())
        .run();
}

//Creating Marker Component for Player
struct Player {
    velocity: f32,
    direction: Direction,
}

impl Default for Player {
    fn default() -> Self {
        Self {
            velocity: 1.,
            direction: Direction::Up,
        }
    }
}

struct Player2 {
    velocity: f32,
    direction: Direction,
}

impl Default for Player2 {
    fn default() -> Self {
        Self {
            direction: Direction::Up,
            velocity: 1.0,
        }
    }
}

// PartialEq to compare (!=,==, etc)
// Copy to copy value (dir = player.direction)
// Clone for Copy
#[derive(PartialEq, Eq, Copy, Clone)]
enum Direction {
    Left,
    Up,
    Right,
    Down,
}

impl Direction {
    fn opposite(self) -> Self {
        match self {
            Self::Left => Self::Right,
            Self::Right => Self::Left,
            Self::Up => Self::Down,
            Self::Down => Self::Up,
        }
    }
}

//Creating material component for Player
struct Material {
    player1_material: Handle<ColorMaterial>,
    player2_material: Handle<ColorMaterial>,
}

//creating initial setup, with Commands as parameter, commands used as spawner
fn setup(mut commands: Commands, mut material: ResMut<Assets<ColorMaterial>>) {
    // insert (spawn) 2D camera, since this is 2D games we use orthographic camera
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    // spawn the material for player
    commands.insert_resource(Material {
        player1_material: material.add(Color::rgb(0.7, 0.7, 0.7).into()),
        player2_material: material.add(Color::rgb(0.7, 0.7, 0.0).into()),
    });

    //testing rapier
    // Static rigid-body with a cuboid shape.
    // let rigid_body1 = RigidBodyBuilder::new_static();
    // let collider1 = ColliderBuilder::cuboid(5.0, 5.0);
    // commands.spawn().insert_bundle((rigid_body1, collider1));
}

fn spawn_players(mut commands: Commands, materials: Res<Material>) {
    commands
        // here we maake Player as entity
        .spawn_bundle(SpriteBundle {
            material: materials.player1_material.clone(),
            sprite: Sprite::new(Vec2::new(10.0, 10.0)),
            ..Default::default()
        })
        .insert(Player::default());

    commands
        // here we maake Player as entity
        .spawn_bundle(SpriteBundle {
            material: materials.player2_material.clone(),
            sprite: Sprite::new(Vec2::new(10.0, 10.0)),
            ..Default::default()
        })
        .insert(Player2::default())
        .insert(RigidBodyBuilder::new_dynamic())
        // always use half of the sprite size
        .insert(ColliderBuilder::cuboid(5.0, 5.0));
}

// Accessing player position from it's transform (included in spritebundle)
fn player_movement(
    // Taking transform component from player entity
    mut movement_data: Query<(&mut Player, &mut Transform)>,
    time: Res<Time>,
    //mut positions2: Query<&mut Transform, With<Player2>>,
    input: Res<Input<KeyCode>>,
) {
    let mut dir: Direction;
    for (mut player, mut transform) in movement_data.iter_mut() {
        //changing player direction
        if input.pressed(KeyCode::Left) {
            //transform.translation.x -= delta_time * velocity.0;
            dir = Direction::Left;
        } else if input.pressed(KeyCode::Right) {
            // transform.translation.x += delta_time * velocity.0;
            dir = Direction::Right;
        } else if input.pressed(KeyCode::Down) {
            // transform.translation.y -= delta_time * velocity.0;
            dir = Direction::Down;
        } else if input.pressed(KeyCode::Up) {
            // transform.translation.y += delta_time * velocity.0;
            dir = Direction::Up;
        } else {
            dir = player.direction;
        }

        if dir != player.direction.opposite() {
            player.direction = dir;
        }

        let delta_time = time.delta_seconds();
        match player.direction {
            Direction::Up => {
                transform.translation.y += 5. * player.velocity;
            }
            Direction::Down => {
                transform.translation.y -= 5. * player.velocity;
            }
            Direction::Left => {
                transform.translation.x -= 5. * player.velocity;
            }
            Direction::Right => {
                transform.translation.x += 5. * player.velocity;
            }
        }
        //speed up
        if input.just_pressed(KeyCode::W) {
            player.velocity += 0.5;
        }
        //speed down
        if input.just_pressed(KeyCode::S) {
            player.velocity -= 0.5;
        }
        // reset
        if input.pressed(KeyCode::R) {
            transform.translation = Vec3::ZERO;
            player.velocity = 1.;
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
