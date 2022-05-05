pub struct TileMap
{
    file_name: String,
}

impl TileMap
{
    pub fn new(file_name_str: &str) -> TileMap
    {
        return TileMap{file_name: file_name_str.parse().unwrap() };
    }

    pub fn get_file_name(&self) -> &str
    {
        return self.file_name.as_ref();
    }
}