#[derive(Clone, Copy)]
pub struct IsoGameGrid
{
    cell_height_in_pixels: i32,
    cell_width_in_pixels: i32,
    maximum_x_coordinate: i32,
    maximum_y_coordinate: i32,
}

impl IsoGameGrid
{
    pub fn instantiate_new(cell_height_in_pixels: i32, cell_width_in_pixels: i32, maximum_y_coordinate: i32, maximum_x_coordinate: i32) -> IsoGameGrid
    {
        return IsoGameGrid{cell_height_in_pixels, cell_width_in_pixels, maximum_y_coordinate, maximum_x_coordinate };
    }

    pub fn get_max_y_coordinate(&self) -> i32
    {
        return self.maximum_y_coordinate;
    }

    pub fn get_max_x_coordinate(&self) -> i32
    {
        return self.maximum_x_coordinate;
    }

    pub fn get_cell_height(&self) -> i32
    {
        return self.cell_height_in_pixels;
    }

    pub fn get_cell_width(&self) -> i32
    {
        return self.cell_width_in_pixels;
    }

    pub fn get_pixel_x_coordinate(&self, grid_x: i32, grid_y: i32) -> f32
    {
        let half_cell_width: i32 = self.cell_width_in_pixels / 2;

        return ((grid_x - grid_y) * half_cell_width) as f32;
    }

    pub fn get_pixel_y_coordinate(&self, grid_x: i32, grid_y: i32) -> f32
    {
        let half_cell_height: i32 = self.cell_height_in_pixels / 2;

        return ((grid_x + grid_y) * half_cell_height) as f32;
    }

    pub fn get_grid_x_coordinate(&self, pixel_x: i32, pixel_y: i32) -> i32
    {
        let half_cell_width: i32 = self.cell_width_in_pixels / 2;
        let half_cell_height: i32 = self.cell_height_in_pixels / 2;

        return ((pixel_x / half_cell_width) + (pixel_y / half_cell_height)) / 2;
    }

    pub fn get_grid_y_coordinate(&self, pixel_x: i32, pixel_y: i32) -> i32
    {
        let half_cell_width: i32 = self.cell_width_in_pixels / 2;
        let half_cell_height: i32 = self.cell_height_in_pixels / 2;

        return ((pixel_y / half_cell_height) - (pixel_x / half_cell_width)) / 2;
    }

    pub fn are_coordinates_legal(&self, grid_x: i32, grid_y: i32) -> bool
    {
        if grid_x < 0 || grid_x > self.maximum_x_coordinate
        {
            return false;
        }

        if grid_y < 0 || grid_y > self.maximum_y_coordinate
        {
            return false;
        }

        return true;
    }

    pub fn find_nearest_legal_x_coordinate(&self, grid_x: i32) -> i32
    {
        if grid_x > self.maximum_x_coordinate
        {
            return self.maximum_x_coordinate;
        }

        if grid_x < 0
        {
            return 0;
        }

        return grid_x;
    }

    pub fn find_nearest_legal_y_coordinate(&self, grid_y: i32) -> i32
    {
        if grid_y > self.maximum_y_coordinate
        {
            return self.maximum_y_coordinate;
        }

        if grid_y < 0
        {
            return 0;
        }

        return grid_y;
    }
}