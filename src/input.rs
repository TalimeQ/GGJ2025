use bevy::{prelude::*, window::CursorGrabMode};
use bevy::input::mouse::MouseMotion;
use bevy::window::PrimaryWindow;


// #[derive(Resource)]
// pub struct MouseData
// {
//     last_mouse_pos: (f32, f32),
// }

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
fn mouse_click_system(mouse_button_input: Res<ButtonInput<MouseButton>>) {
    if mouse_button_input.pressed(MouseButton::Left) {
        info!("left mouse currently pressed");
    }

    if mouse_button_input.just_pressed(MouseButton::Left) {
        info!("left mouse just pressed");
    }

    if mouse_button_input.just_released(MouseButton::Left) {
        info!("left mouse just released");
    }
}

pub fn cursor_position(
    q_windows: Query<&Window, With<PrimaryWindow>>,

) {
    // Games typically only have one window (the primary window)
    if let Some(position) = q_windows.single().cursor_position()
    {
     //   mouse_data.last_mouse_pos = (position.x, position.y);
        println!("Cursor is inside the primary window, at {:?}", position);
    } else {
        println!("Cursor is not in the game window.");
    }
}