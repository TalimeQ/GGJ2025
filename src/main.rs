mod generator;
mod game_state;
mod input;
mod timer;
mod cells;
mod gameui;
mod game_data;

use std::collections::HashMap;
use std::time::Duration;
use bevy::diagnostic::LogDiagnosticsPlugin;
use bevy::prelude::*;
use bevy_asset_loader::prelude::*;
use crate::cells::{cells_system, update_effects, CellSpriteSheet};
use crate::game_data::{game_loop_system, GameData};
use crate::game_state::*;
use crate::gameui::spawn_ui;
use crate::generator::*;
use crate::input::{cursor_position, equip_magic_items, grab_mouse, mouse_click_system, MouseData};
use crate::timer::{GameIterationTimer};

#[derive(Resource, Default)]
pub struct GridConstants
{
    offset : u32,
    x_max : i32,
    y_max : i32
}

fn main()
{
    App::new()
        .add_plugins((DefaultPlugins.set(ImagePlugin::default_nearest())))
        .add_plugins(LogDiagnosticsPlugin::default())
        .init_state::<GameStates>()
        .init_resource::<GridConstants>()
        .init_resource::<MouseData>()
        .init_resource::<CellSpriteSheet>()
        .init_resource::<crate::gameui::UiWrapperSheet>()
        .init_resource::<GameData>()
        .insert_resource::<GameIterationTimer>(GameIterationTimer{
            timer: Timer::new(Duration::from_millis(800), TimerMode::Repeating),
            active: true,
        })
        .add_loading_state(
            LoadingState::new(GameStates::AssetLoading)
                .continue_to_state(GameStates::Next)
                .load_collection::<MapSource>())
        .add_systems(OnEnter(GameStates::Next), (spawn_ui,initialize_grid).chain())
        .add_systems(Update, (cursor_position, grab_mouse, mouse_click_system,
                              equip_magic_items, cells_system, update_effects,
                              game_loop_system).chain())
        .run();
}

