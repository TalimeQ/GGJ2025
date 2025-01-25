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

#[derive(Resource)]
struct CellSpriteSheet;

impl FromWorld for CellSpriteSheet {
    fn from_world(world: &mut World) -> Self {
        let texture_atlas = TextureAtlasLayout::from_grid(
            (24, 24).into(), // The size of each image
            7,               // The number of columns
            1,               // The number of rows
            None,            // Padding
            None,            // Offset
        );

        let mut texture_atlases = world
            .get_resource_mut::<Assets<TextureAtlasLayout>>()
            .unwrap();
        let texture_atlas_handle = texture_atlases.add(texture_atlas);
        Self(texture_atlas_handle)
    }
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
    x : u32,
    y : u32,
    cell_pow : i32,
    neighbors_pow : i32
}

pub fn cells_system(mut query: Query<(&mut Cell)>, data: Res<GridConstants>)
{
    // Each day we strafe further away from god
    for [cell1, cell2] in query.iter_combinations_mut()
    {
        // TODO REFACTOR
        if cell1.x == cell2.x && (cell1.y == cell2.y + 1 ||  cell1.y == cell2.y - 1)
        {
            cell1.neighbors_pow += cell2.cell_pow;
        }
        else if cell1.y == cell2.y && (cell1.x == cell2.x + 1 ||  cell1.x == cell2.x - 1)
        {
            cell1.neighbors_pow += cell2.cell_pow;
        }
        else if cell1.y == cell2.y + 1 && (cell1.x == cell2.x - 1 ||  cell1.x == cell2.x + 1)
        {
            cell1.neighbors_pow += cell2.cell_pow;
        }
        else if cell1.y == cell2.y + 1 && (cell1.x == cell2.x - 1 ||  cell1.x == cell2.x + 1 )
        {
            cell1.neighbors_pow += cell2.cell_pow;
        }
    }

    for cell in query.iter_mut()
    {
        // i wont spend million years on searching for math lib
        cell.cell_pow += cell.neighbors_pow.signum() as i32 ;
        cell.neighbors_pow = 0;
        //call function to reassign cell power
    }
}

fn update_effects(
    mut query: Query<(Cell,&mut Sprite)>)
{
    for (cell ,  sprite) in query.iter_mut()
    {
        if cell.cell_pow > 0
        {

        }
        else if cell.cell_pow < 0
        {

        }
        else
        {

        }
    }
}

// Component initialization example
fn initialize_grid(mut commands: Commands, asset_server: Res<AssetServer>, mut data : ResMut<GridConstants>)
{
    data.offset = commands.spawn(Camera2d).id().index() + 1;

    let sprite_size :f32 = 16.;

    let x_amount : i32 =  40;
    let y_amount : i32 =  20;

    let x_offset : f32 = x_amount as f32 / 2. * sprite_size;
    let y_offset : f32 = y_amount as f32 / 2. * sprite_size;

    data.y_max = y_amount;
    data.x_max = x_amount;

    let mut cell_hashmap  = HashMap::new();

    cell_hashmap.insert(1, CellDefinition{cell_type : CellType::Empty, sprite_path: "sprites/EvilBubble.png".to_string()});

    let  mut entity_id : Entity;

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

