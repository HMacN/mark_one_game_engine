#![forbid(unsafe_code)]

use materiel::mark_one_game_engine::mark_one_game_core::MarkOneGameCore;

fn main()
{
/*    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(IsoGameGrid::instantiate_new(64, 128, 5, 5))
        .insert_resource(SpriteTracker::instantiate_new())
        .insert_resource(UIDTracker::instantiate_new())
        .add_startup_system(setup)
        .run();*/

    //let sprite = materiel::mark_one_game_engine::win_it_boundary_event_loop::WinItBoundaryEventLoop::new().unwrap();

    // sprite.render_window_to_screen("Hugh's Window 31-08-22".to_string());

    let mut core: MarkOneGameCore = MarkOneGameCore::new();

    core.display_window();
}