mod tutorial_module;

use bevy::prelude::*;
use materiel::isometric_game_grid::IsoGameGrid;
use materiel::sprite_bundle_manager::SpriteBundleManager;
use materiel::uid_tracker::UIDTracker;

fn main()
{
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(IsoGameGrid::instantiate_new(64, 128, 5, 5))
        .insert_resource(SpriteBundleManager::instantiate_new())
        .insert_resource(UIDTracker::instantiate_new())
        .add_startup_system(setup)
        .run();
}

fn setup(mut commands: Commands)
{
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
}