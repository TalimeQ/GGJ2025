use bevy::prelude::*;
use crate::{Cell, GridConstants};
use crate::generator::MapSource;

#[derive(Default)]
enum InputState
{
    #[default]
    Available,
    Blocked
}

#[derive(Resource,Default)]
pub struct GameData
{
    percent_filled : f32,
    enemy_filled_win: f32,
    player_filled_win: f32,
    player_currency : i32,
    currency_per_tick: i32,
    input_state: InputState
}

impl Default for GameData
{
    fn default() -> GameData
    {
        GameData
        {
            percent_filled: 0.,
            enemy_filled_win : 0.8,
            player_filled_win : 0.2,
            player_currency : 10,
            currency_per_tick: 10,
            input_state : InputState::Available
        }
    }
}
#[derive(Default)]
pub struct PlayerConsts
{
    empty_cell : i32,
    occupied_cell : i32,
    unit : i32,
    cannon : i32,
    bomb_small : i32,
    bomb_big : i32
}

impl Default for PlayerConsts
{
    fn default() -> PlayerConsts
    {
        PlayerConsts
        {
            empty_cell : 0,
            occupied_cell : 1,
            unit : 3,
            cannon : 10,
            bomb_small : 15,
            bomb_big : 30
        }
    }
}

pub fn game_loop_system
(
    query: Query<Cell>,
    mut game_data: Res<GameData>,
    map_data : Res<GridConstants>
)
{
    let num_cells : i32 = map_data.y_max * map_data.x_max;
    let mut num_enemies : i32 = 0;

    for cell in query.iter()
    {
        if(cell.cell_pow > 0)
        {
            num_enemies += 1;
        }
    }

    game_data.percent_filled = num_enemies as f32 / num_cells as f32;

    if game_data.percent_filled >= game_data.enemy_filled_win
    {
        // Enemy won!
    }
    else if game_data.percent_filled <= game_data.player_filled_win
    {
        // Player won
    }

    // regardless add resources
    game_data.player_currency += game_data.currency_per_tick;
}