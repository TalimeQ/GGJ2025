use bevy::{prelude::*, window::CursorGrabMode};
use bevy::input::mouse::MouseMotion;
use bevy::text::cosmic_text::SwashContent::Color;
use bevy::window::PrimaryWindow;
use crate::{Cell, CellType};
use crate::generator::MainCamera;

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
    mouse_data: Res<MouseData>)
{
    if mouse_button_input.pressed(MouseButton::Left) {
        println!("left mouse currently pressed x: {x}, y: {y}", x = mouse_data.last_mouse_pos.0, y = mouse_data.last_mouse_pos.1);
    }

    if mouse_button_input.just_pressed(MouseButton::Left) {
        println!("left mouse just pressed x: {x}, y: {y}", x = mouse_data.last_mouse_pos.0, y = mouse_data.last_mouse_pos.1);
    }

    if mouse_button_input.just_released(MouseButton::Left) {
        println!("left mouse just released x: {x}, y: {y}", x = mouse_data.last_mouse_pos.0, y = mouse_data.last_mouse_pos.1);
    }
}

pub fn cursor_position(
    mut commands: Commands,
    q_windows: Query<&Window, With<PrimaryWindow>>,
    //asset_server: Res<AssetServer>,
    mut mouse_data: ResMut<MouseData>
) {
    // if let Some(position) = q_windows.single().cursor_position()
    // {
    //     mouse_data.last_mouse_pos = (position.x, position.y);
    //     commands.spawn((Sprite::from_color(Srgba::rgb(1., 0., 0.), Vec2::new(16., 16.)), //asset_server.load("sprites/EvilBubble.png")
    //                     Transform::from_xyz(position.x, position.y, 0.0),
    //                     Cell{cell_type : CellType::BasicPlayer, x: position.x as i32, y: position.y as i32 }));
    //    // println!("Cursor is inside the primary window, at {:?}", position);
    // } else {
    //  //   println!("Cursor is not in the game window.");
    // }
}