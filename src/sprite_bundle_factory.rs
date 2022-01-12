use bevy::asset::AssetPath;
use::bevy::prelude::*;
use bevy::sprite::SpriteBundle;
use crate::in_game_location::InGameLocation;
use crate::isometric_game_grid::IsoGameGrid;

pub fn make_new_sprite_bundle(
        sprite_to_load: AvailableSprites,
        x_coord: i32,
        y_coord: i32,
        grid: &Res<IsoGameGrid>,
        asset_server: &Res<AssetServer>,
    )-> SpriteBundle
{
    let location = InGameLocation::instantiate_new(x_coord, y_coord);
    let new_transform = location.find_transform(**grid);

    return SpriteBundle
    {
        sprite: Default::default(),
        transform: new_transform,
        global_transform: Default::default(),
        texture: asset_server.load(get_asset_path(sprite_to_load)),
        visibility: Default::default()
    }
}

fn get_asset_path(sprite_to_get_path_for: AvailableSprites) -> bevy::asset::AssetPath<'static>
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

#[derive(PartialEq)]
pub enum AvailableSprites
{
    Water,
    Grass,
    Hills,
}