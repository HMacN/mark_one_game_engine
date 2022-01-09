pub struct IsoGameGrid
{
    cell_height_in_pixels: i32,
    cell_width_in_pixels: i32,
    number_of_rows: i32,
    number_of_columns: i32
}

impl IsoGameGrid
{
    pub fn instantiate_new(cell_height_in_pixels: i32, cell_width_in_pixels: i32, number_of_rows: i32, number_of_columns: i32) -> IsoGameGrid
    {
        return IsoGameGrid{cell_height_in_pixels, cell_width_in_pixels, number_of_rows, number_of_columns};
    }

    pub fn get_number_of_rows(&self) -> i32
    {
        return self.number_of_rows;
    }

    pub fn get_number_of_columns(&self) -> i32
    {
        return self.number_of_columns;
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
        if grid_x < 0 || grid_x > self.number_of_columns
        {
            return false;
        }

        if grid_y < 0 || grid_y > self.number_of_rows
        {
            return false;
        }

        return true;
    }

    pub fn find_nearest_legal_x_coordinate(&self, grid_x: i32) -> i32
    {
        if grid_x > self.number_of_columns
        {
            return self.number_of_columns;
        }

        if grid_x < 0
        {
            return 0;
        }

        return grid_x;
    }

    pub fn find_nearest_legal_y_coordinate(&self, grid_y: i32) -> i32
    {
        if grid_y > self.number_of_rows
        {
            return self.number_of_rows;
        }

        if grid_y < 0
        {
            return 0;
        }

        return grid_y;
    }
}

