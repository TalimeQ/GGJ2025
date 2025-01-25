use crate::cells::Cell;
use crate::cells::*;
use crate::generator::SPRITE_SIZE;
use bevy::input::mouse::MouseMotion;
use bevy::window::PrimaryWindow;
use bevy::{prelude::*, window::CursorGrabMode};
use crate::game_data::GameData;
use crate::timer::GameIterationTimer;

enum MagicItem {
    PiuPiuPiu(i32),
    Piuuum(i32, f32), //radius
    Wololo(i32, f32), // radius
    KaBum(i32, f32),  //radius
}

impl Default for MagicItem {
    fn default() -> Self {
        Self::PiuPiuPiu(1)
    }
}

#[derive(Resource, Default)]
pub struct MouseData {
    last_mouse_pos: (f32, f32),
    equipped_magic_item: MagicItem,
}

pub fn grab_mouse(
    mut window: Single<&mut Window>,
    mouse: Res<ButtonInput<MouseButton>>,
    key: Res<ButtonInput<KeyCode>>,
    events: EventReader<MouseMotion>,
) {
    if mouse.just_pressed(MouseButton::Left) {
        window.cursor_options.visible = true;
        window.cursor_options.grab_mode = CursorGrabMode::Locked;
    }

    if key.just_pressed(KeyCode::Escape) {
        window.cursor_options.visible = true;
        window.cursor_options.grab_mode = CursorGrabMode::None;
    }
}

// TODO :: Implement
pub fn mouse_click_system(
    mouse_button_input: Res<ButtonInput<MouseButton>>,
    mouse_data: Res<MouseData>,
    q_cells: Query<(&mut Cell, &mut Transform)>,
    mut game_data: ResMut<GameData>,
) {
    if mouse_button_input.just_pressed(MouseButton::Left) {

        let cost = match mouse_data.equipped_magic_item {
            MagicItem::PiuPiuPiu(cost) => spawn_piupiupiu(
                cost,
                q_cells,
                mouse_data.last_mouse_pos,
                game_data.player_currency),
            MagicItem::Piuuum(cost, range) => spawn_piuuum(
                cost,
                range,
                q_cells,
                mouse_data.last_mouse_pos,
                game_data.player_currency
            ),
            MagicItem::KaBum(cost, range) => spawn_piuuum(
                cost,
                range,
                q_cells,
                mouse_data.last_mouse_pos,
                game_data.player_currency
            ),
            _ => return,
        };
        game_data.player_currency -= cost;
    }
}

pub fn cursor_position(
    q_windows: Query<&Window, With<PrimaryWindow>>,
    mut mouse_data: ResMut<MouseData>,
) {
    if let Some(position) = q_windows.single().cursor_position() {
        let width = q_windows.single().width() / 2.;
        let height = q_windows.single().height() / 2.;

        mouse_data.last_mouse_pos = (
            ((position.x / SPRITE_SIZE) as u32) as f32 * SPRITE_SIZE - width + SPRITE_SIZE / 2.0,
            -(((position.y / SPRITE_SIZE) as u32) as f32 * SPRITE_SIZE - height
                + SPRITE_SIZE / 2.0),
        );
    } else {
        //   println!("Cursor is not in the game window.");
    }
}

fn spawn_kabum(
    magic_item_cost: i32,
    magic_item_range: f32,
    q_cells: Query<(&mut Cell, &mut Transform)>,
    mouse_data: (f32, f32),
    player_currency: i32) -> i32
{
    if magic_item_cost > player_currency
    {
        return 0;
    }
    find_and_spawn(q_cells, mouse_data, CellType::KaBum, magic_item_range);

    magic_item_cost
}

fn spawn_piuuum(
    magic_item_cost: i32,
    magic_item_range: f32,
    q_cells: Query<(&mut Cell, &mut Transform)>,
    mouse_data: (f32, f32),
    player_currency: i32) -> i32
{
    if magic_item_cost > player_currency
    {
        return 0;
    }
    find_and_spawn(q_cells, mouse_data, CellType::Piuuum, magic_item_range);

    magic_item_cost
}

fn spawn_piupiupiu(
    magic_item_cost: i32,
    q_cells: Query<(&mut Cell, &mut Transform)>,
    mouse_data: (f32, f32),
    player_currency: i32) -> i32
{
    if magic_item_cost > player_currency
    {
        return 0;
    }

    find_and_spawn(q_cells, mouse_data, CellType::BasicPlayer, 1.);

    magic_item_cost
}

fn find_and_spawn(
    mut q_cells: Query<(&mut Cell, &mut Transform)>,
    mouse_data: (f32, f32),
    cell_type: CellType,
    power: f32)
{
    let mut cos = q_cells.iter_mut().find_map(|cell| {
        if cell.1.translation.x == mouse_data.0
            && cell.1.translation.y == mouse_data.1
        {
            Some(cell)
        } else {
            None
        }
    });

    if let Some((mut test, mut transform)) = cos {
        transform.translation.x = mouse_data.0;
        transform.translation.y = mouse_data.1;
        test.cell_type = cell_type;
        test.cell_pow = power as i32;
    }
}

pub fn equip_magic_items(
    q_key: Res<ButtonInput<KeyCode>>,
    mut data: ResMut<MouseData>,
    mut timer: ResMut<GameIterationTimer>)
{
    if q_key.just_pressed(KeyCode::Space) {
        timer.active = !timer.active;
    } else if q_key.just_pressed(KeyCode::Digit1) {
        data.equipped_magic_item = MagicItem::PiuPiuPiu(1);
    } else if q_key.just_pressed(KeyCode::Digit2) {
        data.equipped_magic_item = MagicItem::Piuuum(5, 1.);
    } else if q_key.just_pressed(KeyCode::Digit3) {
        data.equipped_magic_item = MagicItem::Wololo(20, 3.);
    } else if q_key.just_pressed(KeyCode::Digit4) {
        data.equipped_magic_item = MagicItem::KaBum(25, 5.);
    }
}
