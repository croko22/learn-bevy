use bevy::prelude::*;

pub mod components;
mod systems;

use systems::*;

use super::SimulationState;
use crate::AppState;

#[derive(SystemSet, Debug, Clone, Hash, PartialEq, Eq)]
pub struct MovementSystemSet;

#[derive(SystemSet, Debug, Clone, Hash, PartialEq, Eq)]
pub struct ConfinementSystemSet;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        //? Configure the system set to run before the confinement system set
        app.configure_set(MovementSystemSet.before(ConfinementSystemSet))
            //* On enter state
            .add_system(spawn_player.in_schedule(OnEnter(AppState::Game)))
            //* Systems
            .add_system(
                player_movement
                    .in_set(MovementSystemSet)
                    .run_if(in_state(AppState::Game))
                    .run_if(in_state(SimulationState::Running)),
            )
            .add_system(
                confine_player_movement
                    .in_set(ConfinementSystemSet)
                    .run_if(in_state(AppState::Game))
                    .run_if(in_state(SimulationState::Running)),
            )
            .add_systems(
                (enemy_hit_player, player_hit_star)
                    .in_set(OnUpdate(AppState::Game))
                    .in_set(OnUpdate(SimulationState::Running)),
            )
            .add_system(despawn_player.in_schedule(OnExit(AppState::Game)));
    }
}
