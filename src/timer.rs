use std::time::Duration;
use bevy::prelude::*;

#[derive(Resource)]
pub struct GameIterationTimer
{
    timer: Timer
}

pub fn setup_game_iteration_timer(mut commands: Commands)
{
    commands.insert_resource(GameIterationTimer{
        timer: Timer::new(Duration::from_millis(500), TimerMode::Repeating)
    });
}