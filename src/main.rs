mod generator;
mod game_state;
mod input;
mod timer;
mod cells;
mod gameui;
mod game_data;

use std::time::Duration;
use bevy::diagnostic::LogDiagnosticsPlugin;
use bevy::prelude::*;
use bevy_asset_loader::prelude::*;
use crate::cells::{cells_system, update_effects, CellSpriteSheet};
use crate::game_data::{game_loop_system, GameData, PlayerConsts};
use crate::game_state::*;
use crate::gameui::{spawn_ui, update_gold_tracker};
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

// Audio shitfest
#[derive(Resource)]
pub struct PopSound
{
   pub should_play_pop : bool
}

impl Default for PopSound
{
    fn default() -> PopSound
    {
        PopSound
        {
            should_play_pop : false
        }
    }
}

#[derive(Resource)]
struct SoundHandles {
    one: Handle<AudioSource>,
}

impl FromWorld for SoundHandles {
    fn from_world(world: &mut World) -> Self {
        let asset_server = world.resource::<AssetServer>();
        Self {
            one: asset_server.load("sounds/popsound.ogg"),
        }
    }
}

fn start_music(asset_server: Res<AssetServer>, mut commands: Commands) {
    commands.spawn((
        AudioPlayer::new(
        asset_server.load("sounds/ZbrodniaPrzeciwLudzkosci.ogg")),
        PlaybackSettings::LOOP,
    ));


}

fn play_sound(mut sound_res: ResMut<PopSound>,
              handles_res : Res<SoundHandles>,
              asset_server: Res<AssetServer>,
              mut commands: Commands
)
{
    if sound_res.should_play_pop
    {
        commands.spawn((AudioPlayer::new(handles_res.one.clone()),
                        PlaybackSettings::DESPAWN));
        sound_res.should_play_pop = false;
    }

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
        .init_resource::<crate::gameui::LetterSheet>()
        .init_resource::<crate::gameui::ActiveSheet>()
        .init_resource::<GameData>()
        .init_resource::<PopSound>()
        .init_resource::<SoundHandles>()
        .insert_resource::<GameIterationTimer>(GameIterationTimer{
            timer: Timer::new(Duration::from_millis(800), TimerMode::Repeating),
            active: true,
        })
        .add_loading_state(
            LoadingState::new(GameStates::AssetLoading)
                .continue_to_state(GameStates::Next)
                .load_collection::<MapSource>())
        .add_systems(OnEnter(GameStates::Next), (spawn_ui,start_music,initialize_grid).chain())
        .add_systems(Update, (cursor_position, grab_mouse, mouse_click_system,
                              equip_magic_items, cells_system, update_effects,
                              game_loop_system, update_gold_tracker, play_sound).chain())
        .run();
}

