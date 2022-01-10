extern crate materiel;

use materiel::isometric_game_grid::IsoGameGrid;

#[test]
fn sets_up_correct_number_of_rows()
{
    let num_rows: i32 = 15;
    let test_grid = IsoGameGrid::instantiate_new(10, 20, num_rows, 10);

    assert_eq!(test_grid.get_max_y_coordinate(), num_rows);
}

#[test]
fn sets_up_correct_number_of_columns()
{
    let num_cols: i32 = 15;
    let test_grid = IsoGameGrid::instantiate_new(10, 20, 10, num_cols);

    assert_eq!(test_grid.get_max_x_coordinate(), num_cols)
}

#[test]
fn sets_up_correct_cell_height()
{
    let height: i32 = 15;
    let test_grid = IsoGameGrid::instantiate_new(height, 20, 10, 10);

    assert_eq!(test_grid.get_cell_height(), height);
}

#[test]
fn sets_up_correct_cell_width()
{
    let width: i32 = 15;
    let test_grid = IsoGameGrid::instantiate_new(10, width, 10, 10);

    assert_eq!(test_grid.get_cell_width(), width);
}

#[test]
fn gives_correct_pixel_x_coordinate()
{
    let test_grid = IsoGameGrid::instantiate_new(10, 20, 10, 10);

    assert_eq!(test_grid.get_pixel_x_coordinate(5, 4), 10.0);
}

#[test]
fn gives_correct_pixel_y_coordinate()
{
    let test_grid = IsoGameGrid::instantiate_new(10, 20, 10, 10);

    assert_eq!(test_grid.get_pixel_y_coordinate(5, 4), 45.0)
}

#[test]
fn gives_correct_map_x_coordinate()
{
    let test_grid = IsoGameGrid::instantiate_new(10, 20, 10, 10);

    assert_eq!(test_grid.get_grid_x_coordinate(10, 45), 5);
}

#[test]
fn gives_correct_map_y_coordinate()
{
    let test_grid = IsoGameGrid::instantiate_new(10, 20, 10, 10);

    assert_eq!(test_grid.get_grid_y_coordinate(10, 45), 4);
}
#[test]
fn identifies_if_coordinates_within_given_grid_area()
{
    let test_grid = IsoGameGrid::instantiate_new(10, 20, 10, 10);

    assert!(test_grid.are_coordinates_legal(10, 10));
    assert!(test_grid.are_coordinates_legal(0, 0));
}

#[test]
fn identifies_if_coordinates_outside_given_grid_area()
{
    let test_grid = IsoGameGrid::instantiate_new(10, 20, 10, 10);

    assert_eq!(test_grid.are_coordinates_legal(10, 11), false);
    assert_eq!(test_grid.are_coordinates_legal(-1, -1), false);
}

#[test]
fn provides_nearest_legal_x_coordinate()
{
    let test_grid = IsoGameGrid::instantiate_new(10, 20, 10, 11);

    assert_eq!(test_grid.find_nearest_legal_x_coordinate(12), 11);
    assert_eq!(test_grid.find_nearest_legal_x_coordinate(-1), 0);
    assert_eq!(test_grid.find_nearest_legal_x_coordinate(0), 0);
}

#[test]
fn provides_nearest_legal_y_coordinate()
{
    let test_grid = IsoGameGrid::instantiate_new(10, 20, 11, 10);

    assert_eq!(test_grid.find_nearest_legal_y_coordinate(12), 11);
    assert_eq!(test_grid.find_nearest_legal_y_coordinate(-1), 0);
    assert_eq!(test_grid.find_nearest_legal_y_coordinate(0), 0);
}