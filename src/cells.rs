use bevy::asset::{Assets, Handle};
use bevy::prelude::{Component, FromWorld, Query, Res, ResMut, Resource, Sprite, TextureAtlasLayout, Time, World};
use crate::{GridConstants};
use crate::game_data::GameData;
use crate::timer::GameIterationTimer;

#[derive(Resource)]
pub struct CellSpriteSheet(pub(crate) Handle<TextureAtlasLayout>);

impl FromWorld for CellSpriteSheet {
    fn from_world(world: &mut World) -> Self {
        let texture_atlas = TextureAtlasLayout::from_grid(
            (16, 16).into(), // The size of each image
            1,               // The number of columns
            5,               // The number of rows
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


// Component examples
#[derive(Clone)]
pub enum CellType
{
    Empty,
    BasicEnemy,
    BasicPlayer,
    Piuuum,
    KaBum
}

pub struct CellDefinition
{
    pub(crate) cell_type : CellType,
    pub(crate) sprite_path: usize
}

#[derive(Component, Clone)]
pub struct Cell
{
    pub(crate) cell_type: CellType,
    pub(crate) x : i32,
    pub(crate) y : i32,
    pub(crate) cell_pow : i32,
    pub(crate) neighbors_pow : i32
}

pub fn cells_system(
    mut query: Query<&mut Cell>,
    data: Res<GridConstants>,
    timer: Res<Time>,
    mut game_iteration_timer: ResMut<GameIterationTimer>,
    mut game_data: ResMut<GameData>)
{
    // Each day we strafe further away from god

    if !game_iteration_timer.active {
        return;
    }

    game_iteration_timer.timer.tick(timer.delta());

    if game_iteration_timer.timer.finished()
    {
        //TODO:: here handle all special cases
        let mut kill_me: Vec<Cell> = Vec::new();
        for cell in query.iter() {
            match cell.cell_type {
                CellType::Piuuum => {
                    let to_kill = query.iter().filter_map(|target|
                        if target.x >= cell.x - cell.cell_pow &&
                            target.x <= cell.x + cell.cell_pow &&
                            target.y >= cell.y - cell.cell_pow &&
                            target.y <= cell.y + cell.cell_pow { Some(target)} else { None });

                    for  target in to_kill {
                        kill_me.push(target.clone());
                    }
                },
                CellType::KaBum => {
                    let to_kill = query.iter().filter_map(|target|
                        if target.x >= cell.x - cell.cell_pow &&
                            target.x <= cell.x + cell.cell_pow &&
                            target.y >= cell.y - cell.cell_pow &&
                            target.y <= cell.y + cell.cell_pow { Some(target)} else { None });

                    for  target in to_kill {
                        kill_me.push(target.clone());
                    }
                },
                _ => continue
            }
        }

        for cell in kill_me{
            let mut to_kill = query.iter_mut().filter_map(|target|
                if target.x == cell.x && target.y == cell.y { Some(target)} else { None });

            if let Some(mut target) = to_kill.next() {
                target.cell_pow = 0;
                target.cell_type = CellType::Empty;
                target.neighbors_pow = 0
            }
        }

        let mut iter = query.iter_combinations_mut();

        while let Some([(mut cell1),(mut cell2)]) = iter.fetch_next()
        {
            calc_neighbor(&mut cell1, &*cell2);
            calc_neighbor(&mut cell2, &*cell1);
        }

        for mut cell in query.iter_mut()
        {
            if cell.neighbors_pow > 0
            {
                if cell.neighbors_pow < 2 && cell.cell_pow !=0
                {
                    cell.cell_pow = 0;
                    cell.cell_type = CellType::Empty;
                }
                else if cell.neighbors_pow == 3 && cell.cell_pow == 0
                {
                    cell.cell_type = CellType::BasicPlayer;
                    cell.cell_pow = 1;
                }
                else if cell.neighbors_pow > 3 && cell.cell_pow !=0
                {
                    cell.cell_pow = 0;
                    cell.cell_type = CellType::BasicPlayer;
                }
            }
            else if cell.neighbors_pow < 0
            {
                if cell.neighbors_pow == -3 && cell.cell_pow == 0
                {
                    cell.cell_type = CellType::BasicEnemy;
                    cell.cell_pow = -1;
                }
                else if cell.neighbors_pow < -3 && cell.cell_pow !=0
                {
                    cell.cell_pow = 0;
                    cell.cell_type = CellType::Empty;
                }
            }
            else {
                cell.cell_type = CellType::Empty;
                cell.cell_pow = 0;
            }

            cell.neighbors_pow = 0;
        }

        // regardless add resources
        game_data.player_currency += game_data.currency_per_tick;
    }
}

fn calc_neighbor(cell1: &mut Cell, cell2: &Cell)
{
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

pub fn update_effects(
    mut query: Query<(&mut Cell,&mut Sprite)>)
{
    for (cell ,  mut sprite) in query.iter_mut()
    {
        let special = match cell.cell_type {
            CellType::Piuuum => {
                if let Some(atlas) = &mut sprite.texture_atlas
                {
                    atlas.index = 4;
                }
                1
            },
            CellType::KaBum => {
                if let Some(atlas) = &mut sprite.texture_atlas
                {
                    atlas.index = 3;
                }
                1
            },
            _ => 0
            };

        if special > 0
        {
            continue;
        }

        if cell.cell_pow > 0
        {
            if let Some(atlas) = &mut sprite.texture_atlas
            {
                atlas.index = 0;
            }
        }
        else if cell.cell_pow < 0
        {
            if let Some(atlas) = &mut sprite.texture_atlas
            {
                atlas.index = 1;
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