mod tutorial_module;

use bevy::prelude::*;
use materiel::isometric_game_grid::IsoGameGrid;

fn main()
{
    App::build()
        .add_plugins(DefaultPlugins)
        .insert_resource(IsoGameGrid::instantiate_new(64, 128, 100, 100))
        .add_startup_system(setup.system())
        .add_system(add_terrain.system())
        .run();
}

fn setup(mut commands: Commands)
{
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
}

fn add_terrain(mut commands: Commands, asset_server: Res<AssetServer>, grid: Res<IsoGameGrid>, mut materials: ResMut<Assets<ColorMaterial>>)
{
    for i in 0..9
    {
        for j in 0..9
        {
            let texture_handle = asset_server.load("Water.png");

            commands.spawn_bundle(SpriteBundle
            {
                material: materials.add(texture_handle.into()),
                transform: Transform::from_xyz(grid.get_pixel_x_coordinate(i, j), grid.get_pixel_y_coordinate(i, j), 0.0),
                ..Default::default()
            });
        }
    }
}

