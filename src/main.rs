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
// struct Player2;

//Creating material component for Player
struct Material(Handle<ColorMaterial>);

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
        .insert(Player);
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
    mut positions: Query<&mut Transform, With<Player>>,
    //mut positions2: Query<&mut Transform, With<Player2>>,
    input: Res<Input<KeyCode>>,
) {
    for mut transform in positions.iter_mut() {
        if input.pressed(KeyCode::Left) {
            transform.translation.x -= 2.;
        }
        if input.pressed(KeyCode::Right) {
            transform.translation.x += 2.;
        }
        if input.pressed(KeyCode::Down) {
            transform.translation.y -= 2.;
        }
        if input.pressed(KeyCode::Up) {
            transform.translation.y += 2.;
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