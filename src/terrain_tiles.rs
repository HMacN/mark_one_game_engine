use bevy::asset::AssetPath;
use bevy::prelude::*;
use crate::in_game_location::InGameLocation;
use crate::isometric_game_grid::IsoGameGrid;

#[derive(Copy, Clone, Component)]
pub struct TerrainTile
{
    terrain_type: TerrainTypes,
}

#[derive(Bundle, Clone)]
pub struct TerrainTileBundle
{
    terrain_type: TerrainTypes,
    location: InGameLocation,

    #[bundle]
    sprite_bundle: SpriteBundle,
}

impl TerrainTileBundle
{
    pub fn instantiate_new(terrain_type: TerrainTypes, x_coord: i32, y_coord: i32, asset_server: &Res<AssetServer>, grid: &Res<IsoGameGrid>) -> TerrainTileBundle
    {
        let location= InGameLocation::instantiate_new(x_coord, y_coord);
        let new_transform = location.find_transform(**grid);

        return TerrainTileBundle{
            terrain_type,
            location,
            sprite_bundle: SpriteBundle
            {
                transform: new_transform,
                texture: asset_server.load(get_path(terrain_type)),
                ..Default::default()
            }
        }
    }
}

fn get_path(terrain_type: TerrainTypes) -> bevy::asset::AssetPath<'static>
{
    return AssetPath::from("Water.png");
}

#[derive(Copy, Clone, Component)]
pub enum TerrainTypes
{
    Water,
    Grass,
    Hills,
}