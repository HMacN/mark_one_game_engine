#![forbid(unsafe_code)]

fn main()
{
/*    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(IsoGameGrid::instantiate_new(64, 128, 5, 5))
        .insert_resource(SpriteTracker::instantiate_new())
        .insert_resource(UIDTracker::instantiate_new())
        .add_startup_system(setup)
        .run();*/

    let sprite = materiel::mark_one_game_engine::display_window::DisplayWindow::new().unwrap();

    sprite.display_window();
}