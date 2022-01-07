mod tutorial_module;

use bevy::asset::AssetPath;
use bevy::prelude::*;
use materiel::new_game_grid;

fn main()
{
    App::build()
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup.system())
        .insert_resource(new_game_grid(64, 128, 100, 100))
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>, mut materials: ResMut<Assets<ColorMaterial>>)
{
    let texture_handle = asset_server.load("Water.png");
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    commands.spawn_bundle(SpriteBundle
    {
        material: materials.add(texture_handle.into()),
        ..Default::default()
    });

    let texture_handle = asset_server.load("Grass.png");

    commands.spawn_bundle(SpriteBundle
    {
        material: materials.add(texture_handle.into()),
        transform: Transform::from_xyz(64.0, 32.0, 0.0),
        ..Default::default()
    });
}
