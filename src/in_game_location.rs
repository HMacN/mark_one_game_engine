use bevy::prelude::*;
use crate::isometric_game_grid::IsoGameGrid;

#[derive(Copy, Clone, Component)]
pub struct InGameLocation
{
    x_coord: i32,
    y_coord: i32,
}

impl InGameLocation
{
    pub fn instantiate_new(starting_x_coord: i32, starting_y_coord: i32) -> InGameLocation
    {
        return InGameLocation{x_coord: starting_x_coord, y_coord: starting_y_coord};
    }

    pub fn get_x_coord(&self) -> i32
    {
        return self.x_coord;
    }

    pub fn get_y_coord(&self) -> i32
    {
        return self.y_coord;
    }

    pub fn find_transform(&self, game_grid: IsoGameGrid) -> Transform
    {
        return game_grid.get_transform_from_grid_coords(self.x_coord, self.y_coord);
    }

    pub fn check_if_legal(&self, game_grid: IsoGameGrid) -> bool
    {
        return if game_grid.are_coordinates_legal(self.x_coord, self.y_coord)
        {
            true
        }
        else
        {
            false
        }
    }

    pub fn make_legal(&mut self, game_grid: IsoGameGrid)
    {
        self.x_coord = game_grid.find_nearest_legal_x_coordinate(self.x_coord);
        self.y_coord = game_grid.find_nearest_legal_y_coordinate(self.y_coord);
    }

    pub fn set_x_coord(&mut self, new_x_coord: i32)
    {
        self.x_coord = new_x_coord;
    }

    pub fn set_y_coord(&mut self, new_y_coord: i32)
    {
        self.y_coord = new_y_coord;
    }
}