mod tutorial_module;

use bevy::prelude::*;
use materiel::isometric_game_grid::IsoGameGrid;
use materiel::terrain_tiles::*;

fn main()
{
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(IsoGameGrid::instantiate_new(64, 128, 5, 5))
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
    let mut x = 0;
    while x <= grid.get_max_x_coordinate()
    {
        let mut y = 0;
        while y <= grid.get_max_y_coordinate()
        {
            commands.spawn_bundle(TerrainTileBundle::instantiate_new(TerrainTypes::Water, x, y, &asset_server, &grid));

            y = y + 1;
        }

        x = x + 1;
    }
}