use bevy::asset::AssetPath;

pub struct SpriteBundleManager
{
    sprite: [AvailableSprites; 1000],
    x_coord: [f32; 1000],
    y_coord: [f32; 1000],
    uid: [u16; 1000],

    next_free_index: usize,
}

impl SpriteBundleManager
{
    pub fn instantiate_new() -> SpriteBundleManager
    {
        return SpriteBundleManager {
            sprite: [AvailableSprites::ImageNotFound; 1000],
            x_coord: [0.0; 1000],
            y_coord: [0.0; 1000],
            uid: [0; 1000],

            next_free_index: 0,
        };
    }

    pub fn add_sprite_bundle(&mut self, sprite: AvailableSprites, screen_x_coord: f32, screen_y_coord: f32, given_uid: u16) -> bool
    {
        if self.uid.contains(&given_uid)
        {
            return false;
        }

        self.sprite[self.next_free_index] = sprite;
        self.x_coord[self.next_free_index] = screen_x_coord;
        self.y_coord[self.next_free_index] = screen_y_coord;
        self.uid[self.next_free_index] = given_uid;

        self.next_free_index = self.next_free_index + 1;

        return true;
    }
    
    pub fn get_details_by_uid(&self, uid: u16) -> (AvailableSprites, f32, f32)
    {
        let index_search_results = self.find_array_index_by_uid(uid);

        if index_search_results.1 == false
        {
            return (AvailableSprites::ImageNotFound, 0.0, 0.0);
        }

        return (self.sprite[index_search_results.0], self.x_coord[index_search_results.0], self.y_coord[index_search_results.0]);
    }

    fn find_array_index_by_uid(&self, given_uid: u16) -> (usize, bool)
    {
        for _i in 0..1000
        {
            if self.uid[_i] == given_uid
            {
                return (_i, true);
            }
        }

        return (0, false);
    }
}


pub(crate) fn get_texture<'a>(sprite_to_get_path_for: AvailableSprites) -> AssetPath<'a>
{
    return if sprite_to_get_path_for == AvailableSprites::Water
    {
        AssetPath::from("Water.png")
    }
    else if sprite_to_get_path_for == AvailableSprites::Grass
    {
        AssetPath::from("Grass.png")
    }
    else if sprite_to_get_path_for == AvailableSprites::Hills
    {
        AssetPath::from("Hills.png")
    }
    else
    {
        AssetPath::from("ImageNotFound.png")
    }
}

#[derive(PartialEq, Copy, Clone, Debug)]
pub enum AvailableSprites
{
    Water,
    Grass,
    Hills,
    ImageNotFound,
}