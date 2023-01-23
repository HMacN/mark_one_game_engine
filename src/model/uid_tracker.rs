/// A simple tracker which allows Unique Identification numbers to be generated.  All this does is
/// count up through the u16 integers and keep track of the highest one returned so far.  It can be
/// initialised to a given starting value, which may be of use when loading a saved game or similar.
pub struct UIDTracker
{
    /// A u16 which tracks the next UID which can be handed out.
    lowest_free_uid: u16,
}

impl UIDTracker
{
    /// A constructor which creates a default_new UIDTracker and sets the first UID to be returned to be 1.
    ///
    /// #Returns
    ///
    /// * A UIDTracker which will return UIDs starting at 1.
    pub fn instantiate_new() -> UIDTracker
    {
        return UIDTracker { lowest_free_uid: 1 }
    }

    /// A constructor which creates a default_new UIDTracker and sets the first UID to be returned to be the
    /// given starting number.
    ///
    /// #Arguments
    ///
    /// * 'starting_int' - a u16 which will be the number that the returned UIDs start from.
    ///
    /// #Returns
    ///
    /// * A UIDTracker which will return UIDs starting at the given number.
    pub fn instantiate_new_from_starting_int(starting_int: u16) -> UIDTracker
    {
        return UIDTracker { lowest_free_uid: starting_int }
    }

    /// Allows the requesting of default_new UID numbers.
    ///
    /// #Returns
    ///
    /// * A u16 which is the default_new UID number being issued.
    pub fn request_new_uid(&mut self) -> u16
    {
        let uid_to_return = self.lowest_free_uid;

        self.lowest_free_uid = self.lowest_free_uid + 1;

        return uid_to_return;
    }
}