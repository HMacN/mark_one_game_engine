use mark_one_game_engine::model::csv_editor::CSVEditor;
use crate::mark_one_game_engine_test_suite::csv_editor_tests::helper_functions::set_up_test_file;

static TEST_FILE_NAME: &str = "csv_editor_test_file.csv";
static TEST_FILE_ROWS: usize = 4;
static TEST_FILE_COLS: usize = 3;
static TEST_FILE_HEADER: [&str; 3] = ["colA", "colB", "colC"];
static TEST_FILE_ROW_0: [&str; 3] = ["a1", "b1", "c1"];
static TEST_FILE_ROW_1: [&str; 3] = ["a2", "b2", "c2"];
static TEST_FILE_ROW_2: [&str; 3] = ["a3", "b3", "c3"];
static TEST_FILE_ROW_3: [&str; 3] = ["a4", "b4", "c4"];
static TEST_FILE_CONTENTS: [[&str; 3]; 5] = [TEST_FILE_HEADER, TEST_FILE_ROW_0, TEST_FILE_ROW_1, TEST_FILE_ROW_2, TEST_FILE_ROW_3];

#[test]
fn new_editor_created_when_given_a_valid_filename()
{
    assert!(set_up_test_file().is_ok());    //Test only passes if the test file is set up.

    assert!(CSVEditor::new(TEST_FILE_NAME).is_some())
}

#[test]
fn new_editor_not_created_when_given_invalid_filename()
{
    assert!(CSVEditor::new("this_file_should_not_exist_or_it_will_break_the_tests.csv").is_none())
}

#[test]
fn gets_file_rows()
{
    assert!(set_up_test_file().is_ok());    //Test only passes if the test file is set up.

    let mut editor: CSVEditor = CSVEditor::new(TEST_FILE_NAME).unwrap();

    assert_eq!(editor.get_number_of_rows(), TEST_FILE_ROWS);
}

#[test]
fn gets_file_cols()
{
    assert!(set_up_test_file().is_ok());    //Test only passes if the test file is set up.

    let mut editor: CSVEditor = CSVEditor::new(TEST_FILE_NAME).unwrap();

    assert_eq!(editor.get_number_of_columns(), TEST_FILE_COLS);
}

#[test]
fn gets_cell_contents()
{
    assert!(set_up_test_file().is_ok());    //Test only passes if the test file is set up.

    let mut editor: CSVEditor = CSVEditor::new(TEST_FILE_NAME).unwrap();

    assert_eq!(editor.get_cell(1, 1).unwrap(), TEST_FILE_ROW_1[1]);
    assert_eq!(editor.get_cell(0, 0).unwrap(), TEST_FILE_ROW_0[0]);
    assert_eq!(editor.get_cell(3, 2).unwrap(), TEST_FILE_ROW_3[2]);
}

#[test]
fn does_not_return_cell_contents_for_invalid_cell()
{
    assert!(set_up_test_file().is_ok());    //Test only passes if the test file is set up.

    let mut editor: CSVEditor = CSVEditor::new(TEST_FILE_NAME).unwrap();

    assert!(editor.get_cell(3, 3).is_none());
    assert!(editor.get_cell(4, 2).is_none());


    assert!(editor.get_cell(2, 20).is_none());
    assert!(editor.get_cell(20, 2).is_none());
    assert!(editor.get_cell(20, 20).is_none());
}

#[test]
fn gets_header_cell()
{
    assert!(set_up_test_file().is_ok());    //Test only passes if the test file is set up.

    let mut editor: CSVEditor = CSVEditor::new(TEST_FILE_NAME).unwrap();

    assert_eq!(editor.get_header_cell(0).unwrap(), TEST_FILE_HEADER[0]);
    assert_eq!(editor.get_header_cell(2).unwrap(), TEST_FILE_HEADER[2]);
}

#[test]
fn does_not_return_contents_for_invalid_header_cell()
{
    assert!(set_up_test_file().is_ok());    //Test only passes if the test file is set up.

    let mut editor: CSVEditor = CSVEditor::new(TEST_FILE_NAME).unwrap();

    assert!(editor.get_header_cell(9).is_none());
    assert!(editor.get_header_cell(20).is_none());
}

#[test]
fn can_get_file_name()
{
    assert!(set_up_test_file().is_ok());    //Test only passes if the test file is set up.

    let editor: CSVEditor = CSVEditor::new(TEST_FILE_NAME).unwrap();

    assert_eq!(editor.get_file_name(), TEST_FILE_NAME);
}

mod helper_functions
{
    use std::error::Error;
    use std::fs;
    use std::ops::Index;
    use std::path::Path;
    use crate::mark_one_game_engine_test_suite::csv_editor_tests::{TEST_FILE_CONTENTS, TEST_FILE_NAME};

    pub(crate) fn set_up_test_file() -> Result<(), Box<dyn Error>>
    {
        if Path::new(TEST_FILE_NAME).exists()
        {
            if confirm_existing_file_is_correct()
            {
                return Ok(()); //We don't need to change anything.
            }
            else
            {
                fs::remove_file(TEST_FILE_NAME)?;   //Delete the old file before creating a new one.
            }
        }

        let mut writer = csv::Writer::from_path(TEST_FILE_NAME)?;

        //Set up file.
        writer.write_record(&[TEST_FILE_CONTENTS[0][0], TEST_FILE_CONTENTS[0][1], TEST_FILE_CONTENTS[0][2]])?;
        writer.write_record(&[TEST_FILE_CONTENTS[1][0], TEST_FILE_CONTENTS[1][1], TEST_FILE_CONTENTS[1][2]])?;
        writer.write_record(&[TEST_FILE_CONTENTS[2][0], TEST_FILE_CONTENTS[2][1], TEST_FILE_CONTENTS[2][2]])?;
        writer.write_record(&[TEST_FILE_CONTENTS[3][0], TEST_FILE_CONTENTS[3][1], TEST_FILE_CONTENTS[3][2]])?;
        writer.write_record(&[TEST_FILE_CONTENTS[4][0], TEST_FILE_CONTENTS[4][1], TEST_FILE_CONTENTS[4][2]])?;

        writer.flush()?;

        return Ok(());
    }

    fn confirm_existing_file_is_correct() -> bool
    {
        let wrapped_reader = csv::Reader::from_path(TEST_FILE_NAME);

        if wrapped_reader.is_err()
        {
            return false;   //If there's an error opening the file then it's not OK.
        }

        let mut file_reader = wrapped_reader.unwrap();
        let mut file_contents: Vec<String> = vec![];
        let mut vec_of_target_file_contents: Vec<&str> = vec![];

        vec_of_target_file_contents.push(TEST_FILE_CONTENTS[0][0]);
        vec_of_target_file_contents.push(TEST_FILE_CONTENTS[0][1]);
        vec_of_target_file_contents.push(TEST_FILE_CONTENTS[0][2]);
        vec_of_target_file_contents.push(TEST_FILE_CONTENTS[1][0]);
        vec_of_target_file_contents.push(TEST_FILE_CONTENTS[1][1]);
        vec_of_target_file_contents.push(TEST_FILE_CONTENTS[1][2]);
        vec_of_target_file_contents.push(TEST_FILE_CONTENTS[2][0]);
        vec_of_target_file_contents.push(TEST_FILE_CONTENTS[2][1]);
        vec_of_target_file_contents.push(TEST_FILE_CONTENTS[2][2]);
        vec_of_target_file_contents.push(TEST_FILE_CONTENTS[3][0]);
        vec_of_target_file_contents.push(TEST_FILE_CONTENTS[3][1]);
        vec_of_target_file_contents.push(TEST_FILE_CONTENTS[3][2]);
        vec_of_target_file_contents.push(TEST_FILE_CONTENTS[4][0]);
        vec_of_target_file_contents.push(TEST_FILE_CONTENTS[4][1]);
        vec_of_target_file_contents.push(TEST_FILE_CONTENTS[4][2]);

        //Read in header.
        file_contents.push(file_reader.headers().unwrap().index(0).to_string());
        file_contents.push(file_reader.headers().unwrap().index(1).to_string());
        file_contents.push(file_reader.headers().unwrap().index(2).to_string());


        for result in file_reader.records()
        {
            if result.is_err()
            {
                continue;
            }

            let record = result.unwrap();

            file_contents.push(record.index(0).to_string());
            file_contents.push(record.index(1).to_string());
            file_contents.push(record.index(2).to_string());
        }

        if file_contents[0] != TEST_FILE_CONTENTS[0][0]
            || file_contents[1] != TEST_FILE_CONTENTS[0][1]
            || file_contents[2] != TEST_FILE_CONTENTS[0][2]
            || file_contents[3] != TEST_FILE_CONTENTS[1][0]
            || file_contents[4] != TEST_FILE_CONTENTS[1][1]
            || file_contents[5] != TEST_FILE_CONTENTS[1][2]
            || file_contents[6] != TEST_FILE_CONTENTS[2][0]
            || file_contents[7] != TEST_FILE_CONTENTS[2][1]
            || file_contents[8] != TEST_FILE_CONTENTS[2][2]
            || file_contents[9] != TEST_FILE_CONTENTS[3][0]
            || file_contents[10] != TEST_FILE_CONTENTS[3][1]
            || file_contents[11] != TEST_FILE_CONTENTS[3][2]
            || file_contents[12] != TEST_FILE_CONTENTS[4][0]
            || file_contents[13] != TEST_FILE_CONTENTS[4][1]
            || file_contents[14] != TEST_FILE_CONTENTS[4][2]
        {
            return false;    //Not all cells in the file match, so it's not OK.
        }

        return true;    //The file must be correct to get this far.
    }
}