mod tutorial_module;

use bevy::prelude::*;
use materiel::isometric_game_grid::IsoGameGrid;

fn main()
{

    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(IsoGameGrid::instantiate_new(64, 128, 100, 100))
        .add_startup_system(setup)
        .add_system(add_terrain)
        .run();
}

fn setup(mut commands: Commands)
{
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
}

fn add_terrain(mut commands: Commands, asset_server: Res<AssetServer>, grid: Res<IsoGameGrid>)
{
    for i in 0..9
    {
        for j in 0..9
        {
            commands.spawn_bundle(SpriteBundle
            {
                texture: asset_server.load("Water.png"),
                transform: Transform::from_xyz(grid.get_pixel_x_coordinate(i, j), grid.get_pixel_y_coordinate(i, j), 0.0),
                ..Default::default()
            });
        }
    }
}