use crate::mark_one_game_engine::csv_editor::CSVEditor;

/// A Tile Map Object which allows a .csv file to be read in as a map, and then referred to using
/// the coordinates of the tiles.
///
/// This object is built with the idea of tiles being defined by the presence or absence of
/// particular chars in a string.  This should allow each tile to be defined by a single String, and
/// is nice and simple to work with.  More complicated systems may be added in future.
pub struct TileMap
{
    /// A CSVEditor object which is interrogated for information by the tile map.
    csv_editor: CSVEditor
}

impl TileMap
{
    /// A constructor for the tile map, which sets up a CSVEditor to read the actual map file.  If
    /// the CSVEditor cannot be set up for any reason this function will return 'None'.
    ///
    /// #Arguments
    ///
    /// * 'file_name' - a string slice which is the file name of the .csv file which is to be used
    ///                 as a map.
    ///
    /// #Returns
    ///
    /// * an Option<TileMap> which should contain the TileMap created from the given .csv file path.
    pub fn new(file_name: &str) -> Option<TileMap>
    {
        let option_of_csv_editor = CSVEditor::new(file_name);

        if option_of_csv_editor.is_none()
        {
            return None;
        }
        else
        {
            let tile_map = TileMap{csv_editor: option_of_csv_editor.unwrap()};

            return Option::Some(tile_map);
        }
    }

    /// A simple getter for the name of the file used to create this TileMap.
    ///
    /// #Returns
    ///
    /// * A string slice which is the name of the file this map was created from.
    pub fn get_file_name(&self) -> &str
    {
        return self.csv_editor.get_file_name();
    }

    /// A getter which retrieves the String which is used to encode a particular tile.  This returns
    /// an empty string ( "" ) if the co-ordinates given are out of range, or if the tile cannot be
    /// read for any reason.
    ///
    /// #Arguments
    ///
    /// * 'row' - a usize which is the index of the row that the desired cell is located on.
    /// * 'col' - a usize which is the index of the column that the desired cell is located on.
    ///
    /// #Returns
    ///
    /// * A String which is the contents of the tile at the given co-ordinates.
    pub fn get_tile_string(&mut self, row: usize, col: usize) -> String
    {
        let option_of_tile = self.csv_editor.get_cell(row, col);
        let empty_string_slice = "";

        if option_of_tile.is_some()
        {
            return option_of_tile.unwrap();
        }

        return empty_string_slice.to_string();
    }

    /// Determines whether or not a given char is present in the String that represents the tile at
    /// the given co-ordinates.
    ///
    /// This is envisioned as being the primary way in which the Tile Map is queried.
    ///
    /// #Arguments
    ///
    /// * 'row' - a usize which is the index of the row that the desired cell is located on.
    /// * 'col' - a usize which is the index of the column that the desired cell is located on.
    /// * 'char_to_search_for' - a char to try and find at the given tile co-ordinates.
    ///
    /// #Returns
    ///
    /// * A bool which describes whether or not the given char is present at the given co-ordinates.
    pub fn detect_tile_char(&mut self, row: usize, col: usize, char_to_search_for: char) -> bool
    {
        let tile_string = self.get_tile_string(row, col);

        if tile_string.contains(char_to_search_for)
        {
            return true;
        }

        return false;
    }

    /// A getter for the number of columns in the tile map.
    ///
    /// #Returns
    ///
    /// * A usize which is the number of columns in this tile map.
    pub fn get_number_of_columns(&mut self) -> usize
    {
        return self.csv_editor.get_number_of_columns();
    }

    /// A getter for the number of rows in the tile map.
    ///
    /// #Returns
    ///
    /// * A usize which is the number of rows in this tile map.
    pub fn get_number_of_rows(&mut self) -> usize
    {
        return self.csv_editor.get_number_of_rows();
    }
}