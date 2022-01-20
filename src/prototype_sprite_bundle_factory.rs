use bevy::asset::AssetPath;
use::bevy::prelude::*;
use bevy::sprite::SpriteBundle;
use crate::isometric_game_grid::IsoGameGrid;

pub fn make_new_sprite_bundle_at_grid_coords (sprite_to_load: AvailableSprites, x_coord: i32, y_coord: i32, grid: &Res<IsoGameGrid>, asset_server: &Res<AssetServer>) -> SpriteBundle
{
    return SpriteBundle
    {
        sprite: Default::default(),
        transform: grid.get_transform_from_grid_coords(x_coord, y_coord),
        global_transform: Default::default(),
        texture: asset_server.load(get_texture(sprite_to_load)),
        visibility: Default::default(),
    }
}

pub fn make_new_sprite_bundle_at_transform (sprite_to_load: AvailableSprites, transform: Transform, asset_server: &Res<AssetServer>) -> SpriteBundle
{
    let bundle_to_return: SpriteBundle = SpriteBundle
    {
        sprite: Default::default(),
        transform,
        global_transform: Default::default(),
        texture: asset_server.load(get_texture(sprite_to_load)),
        visibility: Default::default(),
    };

    return bundle_to_return;
}
pub fn change_sprite_in_sprite_bundle (mut bundle_to_change: SpriteBundle, new_sprite: AvailableSprites, asset_server: &Res<AssetServer>) -> SpriteBundle
{
    bundle_to_change.texture = asset_server.load(get_texture(new_sprite));

    return bundle_to_change;
}

pub fn change_transform_in_sprite_bundle (mut bundle_to_change: SpriteBundle, new_transform: Transform, ) -> SpriteBundle
{
    bundle_to_change.transform = new_transform;

    return bundle_to_change;
}

fn get_texture<'a>(sprite_to_get_path_for: AvailableSprites) -> AssetPath<'a>
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

#[derive(PartialEq, Copy, Clone)]
pub enum AvailableSprites
{
    Water,
    Grass,
    Hills,
    ImageNotFound,
}