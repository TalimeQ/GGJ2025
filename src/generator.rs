use std::collections::HashMap;
use bevy::asset::{Assets, Handle};
use bevy::color::{Color, Srgba};
use bevy::image::Image;
use bevy::prelude::*;
use bevy_asset_loader::asset_collection::AssetCollection;
use crate::{Cell, CellDefinition, CellType, GridConstants};

#[derive(Resource, AssetCollection)]
pub struct MapSource {
    #[asset(path = "maps/map.png")]
    map_tiles: Handle<Image>,
}

#[derive(Component)]
pub struct MainCamera;

const ENEMY_CELL: &str = "ac3232"; //RED
const EMPTY_CELL: &str = "ffffff"; //WHITE


pub fn initialize_grid(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    map_source: Res<MapSource>,
    images: Res<Assets<Image>>,
    mut data : ResMut<GridConstants>)
{
    data.offset = commands.spawn((Camera2d::default(), MainCamera)).id().index() + 1;
    let map = images.get(&map_source.map_tiles).unwrap();

    let sprite_size :f32 = 16.;

    let x_amount : u32 =  map.width();
    let y_amount : u32 =  map.height();

    let x_offset : f32 = x_amount as f32 / 2. * sprite_size;
    let y_offset : f32 = y_amount as f32 / 2. * sprite_size;

    data.y_max = y_amount as i32;
    data.x_max = x_amount as i32;

    let mut cell_hashmap  = HashMap::new();
    cell_hashmap.insert(ENEMY_CELL, CellDefinition{cell_type : CellType::BasicEnemy, sprite_path: "sprites/EvilBubble.png".to_string()});
    cell_hashmap.insert(EMPTY_CELL, CellDefinition{cell_type : CellType::Empty, sprite_path: "sprites/NoSprite.png".to_string()});

    let mut rng = rand::thread_rng();

    for i in 0..  y_amount
    {
        for j in 0.. x_amount
        {
            let x: f32 = j as f32 * sprite_size - x_offset;
            let y: f32 = -(i as f32 * sprite_size - y_offset);
            let z: f32 = 0.;

            let color : Srgba = map.get_color_at(j, i).unwrap().into();
            match cell_hashmap.get(&color.to_hex().as_str().to_lowercase()[1..])
            {
                Some(cell) => {
                    commands.spawn((Sprite::from_image( asset_server.load(&cell.sprite_path)),
                                    Transform::from_xyz(x,y,z),
                                    Cell{cell_type : cell.cell_type.clone(), x: i as i32, y: j as i32 }));
                }
                None => {}
            }
        }
    }

}