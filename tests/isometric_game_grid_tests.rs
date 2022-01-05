extern crate materiel;

#[test]
fn game_grid_sets_up_correct_number_of_rows()
{
    let num_rows: i32 = 15;
    let test_grid = materiel::new_game_grid(10, 20, num_rows, 10);

    assert_eq!(test_grid.get_number_of_rows(), num_rows);
}

#[test]
fn game_grid_sets_up_correct_number_of_columns()
{
    let num_cols: i32 = 15;
    let test_grid = materiel::new_game_grid(10, 20, 10, num_cols);

    assert_eq!(test_grid.get_number_of_columns(), num_cols)
}

#[test]
fn game_grid_sets_up_correct_cell_height()
{
    let height: i32 = 15;
    let test_grid = materiel::new_game_grid(height, 20, 10, 10);

    assert_eq!(test_grid.get_cell_height(), height);
}

#[test]
fn game_grid_sets_up_correct_cell_width()
{
    let width: i32 = 15;
    let test_grid = materiel::new_game_grid(10, width, 10, 10);

    assert_eq!(test_grid.get_cell_width(), width);
}

#[test]
fn game_grid_gives_correct_pixel_x_coordinate()
{
    let test_grid = materiel::new_game_grid(10, 20, 10, 10);

    assert_eq!(test_grid.get_pixel_x_coordinate(5, 4), 10);
}

#[test]
fn game_grid_gives_correct_pixel_y_coordinate()
{
    let test_grid = materiel::new_game_grid(10, 20, 10, 10);

    assert_eq!(test_grid.get_pixel_y_coordinate(5, 4), 45)
}

#[test]
fn game_grid_gives_correct_map_x_coordinate()
{
    let test_grid = materiel::new_game_grid(10, 20, 10, 10);

    assert_eq!(test_grid.get_grid_x_coordinate(10, 45), 5);
}

#[test]
fn game_grid_gives_correct_map_y_coordinate()
{
    let test_grid = materiel::new_game_grid(10, 20, 10, 10);

    assert_eq!(test_grid.get_grid_y_coordinate(10, 45), 4);
}

