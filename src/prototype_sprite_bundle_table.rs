use crate::prototype_sprite_bundle_factory::AvailableSprites;

pub struct SpriteBundleTable
{
    uid_array: [u16; 1000],
    sprite_bundle_updated_array: [bool; 1000],
    transform_x_coord_array: [f32; 1000],
    transform_y_coord_array: [f32; 1000],
    sprite_to_display_array: [AvailableSprites; 1000],

    entries_count: i32,
    highest_uid: u16
}

impl SpriteBundleTable
{
    pub fn instantiate_new() -> SpriteBundleTable
    {
        return SpriteBundleTable
        {
            uid_array: [0; 1000],
            sprite_bundle_updated_array: [true; 1000],
            transform_x_coord_array: [0.0; 1000],
            transform_y_coord_array: [0.0; 1000],
            sprite_to_display_array: [AvailableSprites::Water; 1000],

            entries_count: 0,
            highest_uid: 0
        }
    }
}