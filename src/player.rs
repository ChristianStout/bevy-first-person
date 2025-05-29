use bevy::prelude::*;

#[derive(Component)]
struct Player;

#[derive(Component)]
struct MovementSpeed(usize);

#[derive(Component)]
struct Position(Vec3);

#[allow(dead_code)]
struct CameraSesitivity(Vec2);

fn spawn_player(mut commands: Commands) {
    commands.spawn((
        Player,
        MovementSpeed(10),
        Position(Vec3::default()),
        Camera3d::default(),
    ));
}

fn player_plugin(app: &mut App) {
    app.add_systems(Startup, spawn_player);
}

