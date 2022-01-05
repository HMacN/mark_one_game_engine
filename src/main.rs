mod tutorial_module;

use bevy::prelude::*;
use materiel::new_game_grid;

fn main()
{
    App::build()
        .add_plugins(DefaultPlugins)
        .insert_resource(new_game_grid(50, 100, 100, 100))
        .run();
}