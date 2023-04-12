use std::time::Duration;

use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

#[derive(Resource)]
struct PipeSpawnTimer {
    timer: Timer,
}

#[derive(Component)]
struct Bird;

#[derive(Component)]
struct Pipe;

fn main() {
    App::new()
        .add_plugin(DefaultPlugins)
        .add_system(controlls)
        .add_startup_system(setup)
        .run()
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2dBundle::default());
    commands
        .spawn(SpriteBundle {
            texture: asset_server.load("sprite/Bird.png"),
            ..default()
        })
        .insert(Brid)
        .insert(RigidBody::default())
        .insert(KinematicCharacterController::default());
    commands.insert_resource(PipeSpawnTimer {
        timer: Timer::new(Duration::from_secs(1), TimerMode::Repeating),
    })
}

fn controlls(
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<&mut KinematicCharacterController, With<Bird>>,
    time: Res<Time>,
) {
    let bird = query.single_mut();

    let jump = Vec2(0, 50);

    if keyboard_input.pressed(KeyCode::Space) {
        bird.translation = Some(jump * time.delta_seconds());
    }
}

fn spawn_pipes(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    spwan_timer: ResMut<PipeSpawnTimer>,
    time: Res<Time>,
) {
    spwan_timer.timer.tick(time.delta_seconds());

    if spwan_timer.timer.finished() {
        commands
            .spawn(asset_server.load("sprite/pipe.jpg"))
            .insert(KinematicCharacterController::default())
            .insert(Pipe);
    }
}

fn move_pipes(mut query: Query<&mut KinematicCharacterController, With<Pipe>>, time: Res<Time>) {
    for pipe in query.iter() {
        pipe.translation = Vec2(-20 * time.delta_seconds(), 0);
    }
}
