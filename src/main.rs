mod generator;
mod game_state;
mod input;
mod timer;

use std::collections::HashMap;
use bevy::prelude::*;
use bevy_asset_loader::prelude::*;
use crate::game_state::*;
use crate::generator::*;
use crate::input::{cursor_position, grab_mouse, mouse_click_system, MouseData};
use crate::timer::setup_game_iteration_timer;

// Component examples
#[derive(Clone)]
pub enum CellType
{
    Empty,
    BasicEnemy,
    BasicPlayer
}

#[derive(Resource)]
struct CellSpriteSheet(Handle<TextureAtlasLayout>);

impl FromWorld for CellSpriteSheet {
    fn from_world(world: &mut World) -> Self {
        let texture_atlas = TextureAtlasLayout::from_grid(
            (16, 16).into(), // The size of each image
            1,               // The number of columns
            3,               // The number of rows
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
    sprite_path: usize
}

#[derive(Component)]
struct Cell
{
    cell_type: CellType,
    x : i32,
    y : i32,
    cell_pow : i32,
    neighbors_pow : i32
}

pub fn cells_system(mut query: Query<&mut Cell>, data: Res<GridConstants>)
{
    // Each day we strafe further away from god

    let mut iter = query.iter_combinations_mut();

    while let Some([(mut cell1),(mut cell2)]) = iter.fetch_next()
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
        else if cell1.x == cell2.x - 1 && (cell1.y == cell2.y + 1 || cell1.y == cell2.y - 1)
        {
            cell1.neighbors_pow += cell2.cell_pow;
        }
        else if cell1.x == cell2.x + 1 && (cell1.y == cell2.y + 1 || cell1.y == cell2.y - 1)
        {
            cell1.neighbors_pow += cell2.cell_pow;
        }


        if(cell1.neighbors_pow > 10)
        {
            cell1.neighbors_pow = 10;
        }
        else if(cell1.neighbors_pow < -10)
        {
            cell1.neighbors_pow = -10;
        }
    }

    for mut cell in query.iter_mut()
    {
        // i wont spend million years on searching for math lib
        cell.cell_pow += cell.neighbors_pow.signum() as i32 ;
        cell.neighbors_pow = 0;
        //call function to reassign cell power
    }
}

fn update_effects(
    mut query: Query<(&mut Cell,&mut Sprite)>)
{
    for (cell ,  mut sprite) in query.iter_mut()
    {
        if cell.cell_pow > 0
        {
            if let Some(atlas) = &mut sprite.texture_atlas
            {
                atlas.index = 1;
            }
        }
        else if cell.cell_pow < 0
        {
            if let Some(atlas) = &mut sprite.texture_atlas
            {
                atlas.index = 0;
            }
        }
        else
        {
            if let Some(atlas) = &mut sprite.texture_atlas
            {
                atlas.index = 2;
            }
        }
    }
}

fn main()
{
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .init_state::<GameStates>()
        .init_resource::<GridConstants>()
        .init_resource::<MouseData>()
        .init_resource::<CellSpriteSheet>()
        .add_loading_state(
            LoadingState::new(GameStates::AssetLoading)
                .continue_to_state(GameStates::Next)
                .load_collection::<MapSource>())
        .add_systems(OnEnter(GameStates::Next), (initialize_grid, setup_game_iteration_timer).chain())
        .add_systems(Update, (grab_mouse, cursor_position, mouse_click_system,cells_system,update_effects).chain())
        .run();
}

