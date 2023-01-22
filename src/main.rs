#![forbid(unsafe_code)]

use mark_one_game_engine::model::mark_one_game_core::MarkOneGameCore;

fn main()
{
    let mut core: MarkOneGameCore = MarkOneGameCore::new();

    core.display_window();
}