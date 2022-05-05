use bevy::prelude::*;
use materiel::bevy_stuff::isometric_game_grid::IsoGameGrid;
use materiel::uid_tracker::UIDTracker;
use materiel::bevy_stuff::sprite_tracker::SpriteTracker;


fn main()
{
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(IsoGameGrid::instantiate_new(64, 128, 5, 5))
        .insert_resource(SpriteTracker::instantiate_new())
        .insert_resource(UIDTracker::instantiate_new())
        .add_startup_system(setup)
        .run();
}

fn setup(mut commands: Commands)
{
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
}