use crate::sprite_bundle_spawner::AvailableSprites;

pub struct SpriteBundleManager
{
    sprite: [AvailableSprites; 1000],
    x_coord: [f32; 1000],
    y_coord: [f32; 1000],
    uid: [u16; 1000],
    is_entry_spawned: [bool; 1000],

    next_free_index: usize,
}

pub struct SpriteBundleManagerEntry
{
    sprite: AvailableSprites,
    x_coord: f32,
    y_coord: f32,
    uid: u16,
}

impl SpriteBundleManagerEntry
{
    pub fn get_sprite(&self) -> AvailableSprites
    {
        return self.sprite;
    }

    pub fn get_x_coord(&self) -> f32
    {
        return self.x_coord;
    }

    pub fn get_y_coord(&self) -> f32
    {
        return self.y_coord;
    }

    pub fn get_uid(&self) -> u16
    {
        return self.uid;
    }

    fn default() -> SpriteBundleManagerEntry
    {
        return SpriteBundleManagerEntry
        {
            sprite: AvailableSprites::ImageNotFound,
            x_coord: 0.0,
            y_coord: 0.0,
            uid: 0
        };
    }
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
            is_entry_spawned: [true; 1000],

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
        self.is_entry_spawned[self.next_free_index] = false;

        self.next_free_index = self.next_free_index + 1;

        return true;
    }
    
    pub fn get_details_by_uid(&self, uid: u16) -> SpriteBundleManagerEntry
    {
        let index_search_results = self.find_array_index_by_uid(uid);

        if index_search_results.1 == false
        {
            return SpriteBundleManagerEntry::default();
        }

        return self.generate_struct_containing_data_entry(index_search_results.0);
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

    pub fn update_details(&mut self, given_uid: u16, new_sprite: AvailableSprites, new_x_coord: f32, new_y_coord: f32) -> bool
    {
        let index_search_results = self.find_array_index_by_uid(given_uid);

        if index_search_results.1 == false
        {
            return false;
        }

        self.sprite[index_search_results.0] = new_sprite;
        self.x_coord[index_search_results.0] = new_x_coord;
        self.y_coord[index_search_results.0] = new_y_coord;

        return true;
    }

    fn generate_struct_containing_data_entry(&self, index: usize) -> SpriteBundleManagerEntry
    {
        return SpriteBundleManagerEntry
        {
            sprite: self.sprite[index],
            x_coord: self.x_coord[index],
            y_coord: self.y_coord[index],
            uid: self.uid[index],
        };
    }

    pub fn find_next_entry_to_spawn(&self) -> (SpriteBundleManagerEntry, bool)
    {
        let are_all_entries_spawned = self.are_all_entries_spawned();

        if are_all_entries_spawned == true
        {
            return (SpriteBundleManagerEntry::default(), false);
        }

        return (self.generate_struct_containing_data_entry(self.get_index_of_next_entry_to_spawn()), true);
    }

    fn get_index_of_next_entry_to_spawn(&self) -> usize
    {
        for _i in 0..1000
        {
            if self.is_entry_spawned[_i] == false
            {
                return _i;
            }
        }

        return 0;
    }

    fn are_all_entries_spawned(&self) -> bool
    {
        if self.is_entry_spawned.contains(&false)
        {
            return false;
        }

        return true;
    }

    pub fn mark_entry_as_spawned(&mut self, given_uid: u16)
    {
        let index_search_result = self.find_array_index_by_uid(given_uid);

        if index_search_result.1
        {
            self.is_entry_spawned[self.find_array_index_by_uid(given_uid).0] = true;
        }
    }
}