use std::fs::File;
use std::ops::Index;
use std::path::Path;
use csv::{Position, Reader, StringRecord};

/// A wrapper for the csv crate, which allows cells in a given .csv file to be directly referenced
/// by their coordinates.
pub struct CSVEditor
{
    /// The file path of the .csv file that this editor is associated with.
    file_name: String,

    /// The "normal" .csv file reader which is used to read from the given file.
    reader: Reader<File>,

    /// The starting position of the reader in the file.  Used to reset the reader before new read
    /// operations.
    reader_starting_position: Position,
}

impl CSVEditor
{
    /// A constructor for the file editor.
    ///
    /// # Arguments
    ///
    /// * 'file_name' - A string slice which contains the file path of the file this editor is to be
    ///                 associated with.
    pub fn new(file_name : &str) -> Option<CSVEditor>
    {
        let attempt_at_reader = csv::Reader::from_path(file_name);

        if Path::new(file_name).exists() && attempt_at_reader.is_ok()
        {
            let new_reader = attempt_at_reader.unwrap();
            let starting_position = new_reader.position().clone();

            let new_editor = CSVEditor
            {
                file_name: file_name.to_string(),
                reader: new_reader,
                reader_starting_position: starting_position
            };

            return Some(new_editor);
        }
        else
        {
            return None;
        }
    }

    /// Resets the reader position to the beginning of the file.
    fn reset_reader_position(&mut self)
    {
        let new_position = self.reader_starting_position.clone();

        let outcome = self.reader.seek(new_position);

        if outcome.is_err()
        {
            println!("An error occurred when resetting the file reader position: {}", outcome.err().unwrap().to_string())
        }
    }

    /// Returns the number of rows in the .csv file.
    ///
    /// # Returns
    ///
    /// * 'number_of_rows' - a usize which is the number of rows in the .csv file.
    pub fn get_number_of_rows(&mut self) -> usize
    {
        self.reset_reader_position();

        let number_of_rows_including_headers: usize = self.reader.records().count();

        return number_of_rows_including_headers - 1;
    }

    /// Returns the number of columns in the .csv file.
    ///
    /// # Returns
    ///
    /// * 'number_of_columns' - a usize which is the number of columns in the .csv file.
    pub fn get_number_of_columns(&mut self) -> usize
    {
        self.reset_reader_position();

        let headers_string_records = self.reader.headers();

        if headers_string_records.is_err()
        {
            return 0;   //Return zero if there are no valid headers.
        }

        return headers_string_records.unwrap().len();
    }

    /// Gets the contents of a particular cell at the given coordinates.  Returns 'None' if the
    /// given coordinates are out of range, or if a String cannot be read from the given cell.
    ///
    /// # Arguments
    ///
    /// * 'row_index' -     a usize which is the row the cell to be read is located on.
    /// * 'column_index' -  a usize which is the column the cell to be read is located on.
    ///
    /// # Returns
    ///
    /// * 'cell_contents' - an Option<String> which contains the String read from the given cell.
    pub fn get_cell(&mut self, row_index: usize, col_index: usize) -> Option<String>
    {
        self.reset_reader_position();

        let mut records = self.reader.records();

        let file_line_after_header = row_index + 1;

        let result: Option<Result<StringRecord, csv::Error>> = records.nth(file_line_after_header);

        if result.is_none()
        {
            return None; //Return none if the result for the line is empty.
        }

        let record = result.unwrap();

        if record.is_err()
        {
            return None;    //Return none if the record for this line is empty.
        }

        let line = record.unwrap();

        if line.len() <= col_index
        {
            return None;    //Return none if the column index is out of range.
        }

        return Option::from(line.index(col_index).to_owned());
    }

    /// Gets the contents of a particular cell in the header of the .csv file.  Returns 'None' if
    /// the given cell is out of range for the document, or if a String cannot be read from the
    /// cell.
    ///
    /// # Arguments
    ///
    /// * 'column_index' -  a usize which is the column the cell to be read is located on.
    ///
    /// # Returns
    ///
    /// * 'cell_contents' - an Option<String> which contains the String read from the given cell.
    pub fn get_header_cell(&mut self, col_index: usize) -> Option<String>
    {
        self.reset_reader_position();

        let headers_result = self.reader.headers();

        if headers_result.is_err()
        {
            return None;   //Return none if there are no valid headers.
        }

        let headers = headers_result.unwrap();

        if headers.len() < col_index
        {
            return None;    //Return none if there index is out of range.
        }

        return Option::from(headers.index(col_index).to_owned());
    }

    /// Gets the name of the .csv file that this editor is associated with.
    ///
    /// # Returns
    ///
    /// * 'file_name' - a string slice which is the filename of the .csv file this editor is
    ///                 associated with.
    pub fn get_file_name(&self) -> &str
    {
        return self.file_name.as_str();
    }
}