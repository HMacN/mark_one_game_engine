

pub struct UIDTracker
{
    lowest_free_uid: u16,
}

impl UIDTracker
{
    pub fn instantiate_new() -> UIDTracker
    {
        return UIDTracker { lowest_free_uid: 1 }
    }

    pub fn instantiate_new_from_starting_int(starting_int: u16) -> UIDTracker
    {
        return UIDTracker { lowest_free_uid: starting_int }
    }

    pub fn request_new_uid(&mut self) -> u16
    {
        let uid_to_return = self.lowest_free_uid;

        self.lowest_free_uid = self.lowest_free_uid + 1;

        return uid_to_return;
    }
}