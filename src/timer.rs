use std::time::Duration;
use bevy::prelude::*;

#[derive(Resource)]
pub struct GameIterationTimer
{
    pub(crate) timer: Timer
}