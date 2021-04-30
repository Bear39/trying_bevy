use bevy::prelude::*;
use bevy_rapier2d::{
    physics::{RapierPhysicsPlugin, RigidBodyHandleComponent},
    rapier::{
        dynamics::{RigidBodyBuilder, RigidBodySet},
        geometry::ColliderBuilder,
        na::{Isometry2, Vector2},
    },
    //render::RapierRenderPlugin,
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
        .add_system(player1_movement.system())
        .add_system(player2_movement.system())
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
            velocity: 250.,
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
            velocity: 100.,
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
fn player1_movement(
    // Taking transform component from player entity
    mut movement_data: Query<(&mut Player, &mut Transform)>,
    time: Res<Time>,
    //mut positions2: Query<&mut Transform, With<Player2>>,
    input: Res<Input<KeyCode>>,
) {
    for (mut player, mut transform) in movement_data.iter_mut() {
        let dir: Direction;
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
                transform.translation.y += delta_time * player.velocity;
            }
            Direction::Down => {
                transform.translation.y -= delta_time * player.velocity;
            }
            Direction::Left => {
                transform.translation.x -= delta_time * player.velocity;
            }
            Direction::Right => {
                transform.translation.x += delta_time * player.velocity;
            }
        }
        //speed up
        if input.just_pressed(KeyCode::F1) {
            player.velocity += 10.;
        }
        //speed down
        if input.just_pressed(KeyCode::F2) {
            player.velocity -= 10.;
        }
        // reset
        if input.pressed(KeyCode::Escape) {
            transform.translation = Vec3::ZERO;
            player.velocity = 250.;
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

fn player2_movement(
    input: Res<Input<KeyCode>>,
    mut rigid_body: ResMut<RigidBodySet>,
    mut query: Query<(&mut Player2, &RigidBodyHandleComponent)>,
) {
    for (mut player, rigid_body_component) in query.iter_mut() {
        let dir: Direction;
        if input.pressed(KeyCode::A) {
            //transform.translation.x -= delta_time * velocity.0;
            dir = Direction::Left;
        } else if input.pressed(KeyCode::D) {
            // transform.translation.x += delta_time * velocity.0;
            dir = Direction::Right;
        } else if input.pressed(KeyCode::S) {
            // transform.translation.y -= delta_time * velocity.0;
            dir = Direction::Down;
        } else if input.pressed(KeyCode::W) {
            // transform.translation.y += delta_time * velocity.0;
            dir = Direction::Up;
        } else {
            dir = player.direction;
        }

        if dir != player.direction.opposite() {
            player.direction = dir;
        }

        let x_asis: i8;
        let y_asis: i8;

        match player.direction {
            Direction::Up => {
                x_asis = 0;
                y_asis = 1;
            }
            Direction::Down => {
                x_asis = 0;
                y_asis = -1;
            }
            Direction::Left => {
                x_asis = -1;
                y_asis = 0;
            }
            Direction::Right => {
                x_asis = 1;
                y_asis = 0;
            }
        }

        if input.just_pressed(KeyCode::F3) {
            player.velocity += 1.;
        }
        //speed down
        if input.just_pressed(KeyCode::F4) {
            player.velocity -= 1.;
        }

        let linvel = Vector2::new(
            x_asis as f32 * player.velocity,
            y_asis as f32 * player.velocity,
        );

        if let Some(rb) = rigid_body.get_mut(rigid_body_component.handle()) {
            rb.set_linvel(linvel, true);

            if input.pressed(KeyCode::Escape) {
                rb.set_position(Isometry2::identity(), true);
            }
        }
    }
}
