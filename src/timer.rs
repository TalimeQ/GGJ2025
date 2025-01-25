use bevy::prelude::*;

#[derive(Resource)]
pub struct GameIterationTimer
{
    pub(crate) timer: Timer,
    pub(crate) active: bool
}

impl Default for GameIterationTimer {
    fn default() -> GameIterationTimer {
        GameIterationTimer
        {
            timer: Timer::default(),
            active: true
        }
    }
}