use bevy::asset::{AssetPath, AssetServer};
use bevy::prelude::{Res, Transform};
use bevy::sprite::SpriteBundle;

//Move to BevyBoundary
pub fn make_new_sprite_bundle_at_transform (sprite_to_load: AvailableSprites, transform: Transform, asset_server: &Res<AssetServer>) -> SpriteBundle
{
    let bundle_to_return: SpriteBundle = SpriteBundle
    {
        sprite: Default::default(),
        transform,
        global_transform: Default::default(),
        texture: asset_server.load(find_asset_path(sprite_to_load)),
        visibility: Default::default(),
    };

    return bundle_to_return;
}

//Move this enum and following function into SpriteTracker.
#[derive(PartialEq, Copy, Clone, Debug)]
pub enum AvailableSprites
{
    Water,
    Grass,
    Hills,
    ImageNotFound,
}

fn find_asset_path(sprite: AvailableSprites) -> AssetPath <'static>
{
    use AvailableSprites::*;

    let path_as_str = match sprite
    {
        Water => "sprites/Water.png",
        Grass => "sprites/Grass.png",
        Hills => "sprites/Hills.png",
        ImageNotFound => "sprites/ImageNotFound.png",
    };

    return AssetPath::from(path_as_str);
}