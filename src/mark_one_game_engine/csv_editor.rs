use std::fs::File;
use std::ops::Index;
use std::path::Path;
use csv::{Position, Reader, StringRecord};

pub struct CSVEditor
{
    file_name: String,
    reader: Reader<File>,
    reader_starting_position: Position,
}

impl CSVEditor
{
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

    fn reset_reader_position(&mut self)
    {
        let new_position = self.reader_starting_position.clone();

        let outcome = self.reader.seek(new_position);

        if outcome.is_err()
        {
            println!("An error occurred when resetting the file reader position: {}", outcome.err().unwrap().to_string())
        }
    }

    pub fn get_rows(&mut self) -> usize
    {
        self.reset_reader_position();

        let number_of_rows_including_headers: usize = self.reader.records().count();

        return number_of_rows_including_headers - 1;
    }

    pub fn get_cols(&mut self) -> usize
    {
        self.reset_reader_position();

        let headers_string_records = self.reader.headers();

        if headers_string_records.is_err()
        {
            return 0;   //Return zero if there are no valid headers.
        }

        return headers_string_records.unwrap().len();
    }

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

    pub fn get_file_name(&self) -> &str
    {
        return self.file_name.as_str();
    }
}