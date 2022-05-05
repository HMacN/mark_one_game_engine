use bevy::prelude::Transform;
use materiel::bevy_stuff::in_game_location::InGameLocation;
use materiel::bevy_stuff::isometric_game_grid::IsoGameGrid;

#[test]
fn location_sets_up_correct_x_coord()
{
    let location = InGameLocation::instantiate_new(4, 5);

    assert_eq!(location.get_x_coord(), 4);
}

#[test]
fn location_sets_up_correct_y_coord()
{
    let location = InGameLocation::instantiate_new(4, 5);

    assert_eq!(location.get_y_coord(), 5);
}

#[test]
fn location_can_generate_bevy_transform()
{
    let location = InGameLocation::instantiate_new(4, 5);
    let test_grid = IsoGameGrid::instantiate_new(10, 20, 10, 10);
    let target_transform = Transform::from_xyz(test_grid.get_pixel_x_coordinate(4, 5), test_grid.get_pixel_y_coordinate(4, 5), 0.0);

    assert_eq!(location.find_transform(test_grid), target_transform);
}

#[test]
fn location_checks_if_legal_coordinates()
{
    let location_one = InGameLocation::instantiate_new(-1, -1);
    let location_two = InGameLocation::instantiate_new(11, 11);
    let location_three = InGameLocation::instantiate_new(5, 5);
    let test_grid = IsoGameGrid::instantiate_new(10, 20, 10, 10);

    assert_eq!(location_one.check_if_legal(test_grid), false);
    assert_eq!(location_two.check_if_legal(test_grid), false);
    assert_eq!(location_three.check_if_legal(test_grid), true);
}

#[test]
fn location_changes_to_nearest_legal_coordinates_if_less_than_zero()
{
    let mut location = InGameLocation::instantiate_new(-1, -1);
    let test_grid = IsoGameGrid::instantiate_new(10, 20, 10, 10);

    location.make_legal(test_grid);

    assert_eq!(location.get_x_coord(), 0);
    assert_eq!(location.get_y_coord(), 0);
}

#[test]
fn location_changes_to_nearest_legal_coordinates_if_above_maximum()
{
    let mut location = InGameLocation::instantiate_new(11, 11);
    let test_grid = IsoGameGrid::instantiate_new(10, 20, 10, 10);

    location.make_legal(test_grid);

    assert_eq!(location.get_x_coord(), 10);
    assert_eq!(location.get_y_coord(), 10);
}

#[test]
fn location_does_not_change_if_coordinates_already_legal()
{
    let mut location = InGameLocation::instantiate_new(5, 5);
    let test_grid = IsoGameGrid::instantiate_new(10, 20, 10, 10);

    location.make_legal(test_grid);

    assert_eq!(location.get_x_coord(), 5);
    assert_eq!(location.get_y_coord(), 5);
}

#[test]
fn update_x_coord_correctly()
{
    let mut location = InGameLocation::instantiate_new(5, 5);

    location.set_x_coord(7);

    assert_eq!(location.get_x_coord(), 7);
}

#[test]
fn update_y_coord_correctly()
{
    let mut location = InGameLocation::instantiate_new(5, 5);

    location.set_y_coord(7);

    assert_eq!(location.get_y_coord(), 7);
}