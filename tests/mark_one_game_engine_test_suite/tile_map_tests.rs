use materiel::mark_one_game_engine::tile_map::TileMap;
use std::io;

#[test]
fn can_check_tile_map_filename()
{
    let file_name = "tile_map_test_file";
    let tile_map = TileMap::new(file_name);

    assert_eq!(tile_map.get_file_name(), file_name);
}
