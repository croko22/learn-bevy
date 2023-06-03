pub mod enemy;
mod player;
pub mod score;
mod star;
mod systems;
mod ui;

use bevy::prelude::*;
use systems::*;
use ui::GameUIPlugin;

use crate::events::GameOver;
use crate::AppState;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_state::<SimulationState>()
            //Events
            .add_event::<GameOver>()
            //On Enter State
            .add_system(pause_simulation.in_schedule(OnEnter(AppState::Game)))
            //Plugins
            .add_plugin(enemy::EnemyPlugin)
            .add_plugin(player::PlayerPlugin)
            .add_plugin(score::ScorePlugin)
            .add_plugin(star::StarPlugin)
            .add_plugin(GameUIPlugin)
            //Systems
            .add_system(toggle_simulation.run_if(in_state(AppState::Game)))
            //On Exit State
            .add_system(resume_simulation.in_schedule(OnExit(AppState::Game)));
    }
}

#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
pub enum SimulationState {
    #[default]
    Running,
    Paused,
}
