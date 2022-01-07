use bevy::prelude::*;

pub struct TerrainObjectsArray
{
    array_of_all_game_terrain_objects: [(i32, i32, SpriteToRender, GameObjectType, i32)],
}

pub enum SpriteToRender
{
    Grass(Handle<Sprite>),
    Hills(Handle<Sprite>),
    Water(Handle<Sprite>),
}

pub enum GameObjectType
{
    Terrain,
}

impl TerrainObjectsArray
{
    fn initialise_from_file(file_name: String)
    {

    }
}