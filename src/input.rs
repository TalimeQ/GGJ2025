use bevy::{prelude::*, window::CursorGrabMode};
use bevy::input::mouse::MouseMotion;
use bevy::window::PrimaryWindow;
use crate::{Cell, CellType};
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
    mut commands: Commands,
    mouse_button_input: Res<ButtonInput<MouseButton>>,
    mouse_data: Res<MouseData>)
{
    if mouse_button_input.pressed(MouseButton::Left) {
        println!("left mouse currently pressed x: {x}, y: {y}", x = mouse_data.last_mouse_pos.0, y = mouse_data.last_mouse_pos.1);
    }

    if mouse_button_input.just_pressed(MouseButton::Left) {
        commands.spawn((Sprite::from_color(Srgba::rgb(1., 0., 0.), Vec2::new(16., 16.)), //asset_server.load("sprites/EvilBubble.png")
                        Transform::from_xyz(mouse_data.last_mouse_pos.0, mouse_data.last_mouse_pos.1, 0.0),
                        Cell{cell_type : CellType::BasicPlayer, x: mouse_data.last_mouse_pos.0 as i32, y: mouse_data.last_mouse_pos.1 as i32, cell_pow: 0, neighbors_pow: 0 }));
        println!("left mouse just pressed x: {x}, y: {y}", x = mouse_data.last_mouse_pos.0, y = mouse_data.last_mouse_pos.1);
    }

    if mouse_button_input.just_released(MouseButton::Left) {
        println!("left mouse just released x: {x}, y: {y}", x = mouse_data.last_mouse_pos.0, y = mouse_data.last_mouse_pos.1);
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