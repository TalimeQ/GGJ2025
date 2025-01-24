mod input;

use bevy::prelude::*;
use bevy::window::WindowMode;
use std::collections::HashMap;

// Component examples
#[derive(Clone)]
pub enum CellType
{
    Empty,
    BasicEnemy,
    BasicPlayer
}


pub struct CellDefinition
{
    cell_type : CellType,
    sprite_path: String
}

enum SpritePath
{

}

#[derive(Component)]
struct Cell
{
    cell_type: CellType
}

// Component initialization example
fn initialize_grid(mut commands: Commands, asset_server: Res<AssetServer>)
{
    commands.spawn(Camera2d);

    let sprite_size :f32 = 16.;

    let x_amount : i32 =  40;
    let y_amount : i32 =  20;

    let x_offset : f32 = x_amount as f32 / 2. * sprite_size;
    let y_offset : f32 = y_amount as f32 / 2. * sprite_size;

    let mut cell_hashmap  = HashMap::new();

    cell_hashmap.insert(1, CellDefinition{cell_type : CellType::Empty, sprite_path: "sprites/EvilBubble.png".to_string()});


    let mut rng = rand::thread_rng();

    for i in 0..  y_amount
    {
        for j in 0.. x_amount
        {
            let x: f32 = j as f32 * sprite_size - x_offset;
            let y: f32 = i as f32 * sprite_size - y_offset;
            let z: f32 = 0.;

            match cell_hashmap.get(&1)
            {
                Some(cell) => {
                    commands.spawn((Sprite::from_image( asset_server.load(&cell.sprite_path)),
                                    Transform::from_xyz(x,y,z),
                                    Cell{cell_type : cell.cell_type.clone()}));
                }
                None => {}
            }
        }
    }

}

fn main()
{
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, initialize_grid)
        .add_systems(Update, (input::grab_mouse, input::cursor_position))
        .run();
}

