mod generator;
mod game_state;
mod input;

use std::collections::HashMap;
use bevy::prelude::*;
use bevy_asset_loader::prelude::*;
use crate::game_state::*;
use crate::generator::*;
use crate::input::{cursor_position, grab_mouse, mouse_click_system, MouseData};

// Component examples
#[derive(Clone)]
pub enum CellType
{
    Empty,
    BasicEnemy,
    BasicPlayer
}

#[derive(Resource, Default)]
pub struct GridConstants
{
    offset : u32,
    x_max : i32,
    y_max : i32
}

pub struct CellDefinition
{
    cell_type : CellType,
    sprite_path: String
}

#[derive(Component)]
struct Cell
{
    cell_type: CellType,
    x : i32,
    y : i32
}

pub fn cells_system(mut query: Query<(&mut Cell)>, data: Res<GridConstants>)
{
    // for now its 7
    // store that number as an offset
    // for [cell1, cell2] in query.iter_combinations_mut()
    // {
    //
    // }

    //
}

fn main()
{
    App::new()
        .add_plugins(DefaultPlugins)
        .init_state::<GameStates>()
        .init_resource::<GridConstants>()
        .init_resource::<MouseData>()
        .add_loading_state(
            LoadingState::new(GameStates::AssetLoading)
                .continue_to_state(GameStates::Next)
                .load_collection::<MapSource>())
        .add_systems(OnEnter(GameStates::Next), initialize_grid)
        .add_systems(Update, (grab_mouse, cursor_position, mouse_click_system).chain())
        .run();
}

