use bevy::{prelude::*, window::CursorGrabMode};
use bevy::input::mouse::MouseMotion;
use bevy::window::PrimaryWindow;
use crate::cells::*;
use crate::cells::Cell;
use crate::generator::SPRITE_SIZE;

#[derive(Resource, Default)]
pub struct MouseData
{
    last_mouse_pos: (f32, f32),
}

pub fn grab_mouse(
    mut window: Single<&mut Window>,
    mouse: Res<ButtonInput<MouseButton>>,
    key: Res<ButtonInput<KeyCode>>,
    events : EventReader<MouseMotion>
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
    mut q_cells: Query<(&mut Cell, &mut Transform)>)
{
    if mouse_button_input.just_pressed(MouseButton::Left) {
        let mut cos = q_cells.iter_mut()
            .find_map(|cell| if cell.1.translation.x == mouse_data.last_mouse_pos.0 && cell.1.translation.y ==
            mouse_data.last_mouse_pos.1 { Some(cell) } else {None});

        println!("left mouse currently pressed x: {x}, y: {y}", x = mouse_data.last_mouse_pos.0, y = mouse_data.last_mouse_pos.1);
        if let Some((mut test, mut transform)) = cos {
            transform.translation.x = mouse_data.last_mouse_pos.0;
            transform.translation.y = mouse_data.last_mouse_pos.1;
            test.cell_type = CellType::BasicPlayer;
            test.cell_pow = 1;
        }
    }

    if mouse_button_input.just_released(MouseButton::Left) {
        //println!("left mouse just released x: {x}, y: {y}", x = mouse_data.last_mouse_pos.0, y = mouse_data.last_mouse_pos.1);
    }
}

pub fn cursor_position(
    q_windows: Query<&Window, With<PrimaryWindow>>,
    mut mouse_data: ResMut<MouseData>
) {
    if let Some(position) = q_windows.single().cursor_position()
    {
        let width = q_windows.single().width() / 2.;
        let height = q_windows.single().height() / 2.;

        mouse_data.last_mouse_pos = (((position.x / SPRITE_SIZE) as u32) as f32 * SPRITE_SIZE - width + SPRITE_SIZE / 2.0,
                                     -(((position.y / SPRITE_SIZE) as u32) as f32 * SPRITE_SIZE - height + SPRITE_SIZE / 2.0));
    } else {
     //   println!("Cursor is not in the game window.");
    }
}