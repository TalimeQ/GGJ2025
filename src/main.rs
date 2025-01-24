mod generator;
mod game_state;

use bevy::prelude::*;
use bevy_asset_loader::prelude::*;
use crate::game_state::*;
use crate::generator::*;

// Component examples
enum CellType {
    Empty,
    BasicEnemy,
    BasicPlayer,
}

#[derive(Component)]
struct Cell {
    type_as_int: CellType,
}

// A simple system
fn hello_world() {}

// Component initialization example
fn add_people(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2d);
    commands.spawn((
        Sprite::from_image(asset_server.load("sprites/mushroom.png")),
        Transform::from_xyz(256., 0., 0.),
        Cell {
            type_as_int: CellType::BasicEnemy,
        },
    ));
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .init_state::<GameStates>()
        .add_loading_state(
            LoadingState::new(GameStates::AssetLoading)
                .continue_to_state(GameStates::Next)
                .load_collection::<MapSource>())
        .add_systems(Startup, (add_people).chain())
        .add_systems(OnEnter(GameStates::Next), prepare_map)
        .add_systems(Update, hello_world)
        .run();
}
