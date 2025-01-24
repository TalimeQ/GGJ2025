mod input;

use bevy::prelude::*;


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

// A simple system
fn hello_world()
{

}

// Component initialization example
fn add_people(mut commands: Commands, asset_server: Res<AssetServer>)
{
    commands.spawn(Camera2d);
    commands.spawn((Sprite::from_image( asset_server.load("sprites/mushroom.png")), Transform::from_xyz(256.,0.,0.),Cell{type_as_int: CellType::BasicEnemy}));
}

fn main()
{
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, add_people)
        .add_systems(Update, (hello_world, input::grab_mouse, input::cursor_position))
        .run();
}

