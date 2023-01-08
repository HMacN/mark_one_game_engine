use materiel::model::tile_map::TileMap;

#[test]
fn can_check_tile_map_filename()
{
    let file_name = "tile_map_test_file.csv";
    let tile_map = TileMap::new(file_name).unwrap();

    assert_eq!(tile_map.get_file_name(), file_name);
}

#[test]
fn can_get_string_from_valid_tile()
{
    let file_name = "tile_map_test_file.csv";
    let mut tile_map = TileMap::new(file_name).unwrap();

    assert_eq!(tile_map.get_tile_string(3, 2), "4c");
}

#[test]
fn returns_empty_string_from_invalid_tile()
{
    let file_name = "tile_map_test_file.csv";
    let mut tile_map = TileMap::new(file_name).unwrap();

    assert_eq!(tile_map.get_tile_string(10, 1), "");
    assert_eq!(tile_map.get_tile_string(1, 10), "");
    assert_eq!(tile_map.get_tile_string(3, 3), "");
    assert_eq!(tile_map.get_tile_string(4, 2), "");
}

#[test]
fn detects_char_in_tile()
{
    let file_name = "tile_map_test_file.csv";
    let mut tile_map = TileMap::new(file_name).unwrap();

    assert!(tile_map.detect_tile_char(3, 2, 'c'));
}

#[test]
fn detects_char_not_in_tile()
{
    let file_name = "tile_map_test_file.csv";
    let mut tile_map = TileMap::new(file_name).unwrap();

    assert_eq!(tile_map.detect_tile_char(3, 2, 'd'), false);
}

#[test]
fn gets_number_of_columns()
{
    let file_name = "tile_map_test_file.csv";
    let mut tile_map = TileMap::new(file_name).unwrap();

    assert_eq!(tile_map.get_number_of_columns(), 3);
}

#[test]
fn gets_number_of_rows()
{
    let file_name = "tile_map_test_file.csv";
    let mut tile_map = TileMap::new(file_name).unwrap();

    assert_eq!(tile_map.get_number_of_rows(), 4);
}