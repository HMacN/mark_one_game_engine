use crate::mark_one_game_engine::csv_editor::CSVEditor;

pub struct TileMap
{
    csv_editor: CSVEditor
}

impl TileMap
{
    pub fn new(file_name_str: &str) -> Option<TileMap>
    {
        let option_of_csv_editor = CSVEditor::new(file_name_str);

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

    pub fn get_file_name(&self) -> &str
    {
        return self.csv_editor.get_file_name();
    }

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

    pub fn detect_tile_char(&mut self, row: usize, col: usize, char_to_search_for: char) -> bool
    {
        let tile_string = self.get_tile_string(row, col);

        if tile_string.contains(char_to_search_for)
        {
            return true;
        }

        return false;
    }
}