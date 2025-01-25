mod generator;
mod game_state;
mod input;

use bevy::prelude::*;
use bevy_asset_loader::prelude::*;
use crate::game_state::*;
use crate::generator::*;

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

fn main()
{
    App::new()
        .add_plugins(DefaultPlugins)
        .init_state::<GameStates>()
        .add_loading_state(
            LoadingState::new(GameStates::AssetLoading)
                .continue_to_state(GameStates::Next)
                .load_collection::<MapSource>())
        .add_systems(OnEnter(GameStates::Next), initialize_grid)
        .add_systems(Update, (input::grab_mouse, input::cursor_position))
        .run();
}

