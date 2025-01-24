mod input;

use bevy::prelude::*;
use bevy::window::WindowMode;

// Component examples
enum CellType
{
    Empty,
    BasicEnemy,
    BasicPlayer
}

#[derive(Component)]
struct Cell
{
    type_as_int : CellType,
}

// Component initialization example
fn add_people(mut commands: Commands, asset_server: Res<AssetServer>)
{
    commands.spawn(Camera2d);

    let x_amount : i32 =  40;
    let y_amount : i32 =  20;

    let x_offset : f32 = x_amount as f32 / 2. * 32.;
    let y_offset : f32 = y_amount as f32 / 2. * 32.;

    for i in 0..  y_amount
    {
        for j in 0.. x_amount
        {
            let x: f32 = j as f32 * 32. - x_offset;
            let y: f32 = i as f32 * 32. - y_offset;
            let z: f32 = 0.;
            commands.spawn((Sprite::from_image( asset_server.load("sprites/mushroom.png")), Transform::from_xyz(x,y,z),Cell{type_as_int: CellType::BasicEnemy}));
        }
    }

}

fn main()
{
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, add_people)
        .add_systems(Update, (input::grab_mouse, input::cursor_position))
        .run();
}

