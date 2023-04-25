use bevy::prelude::*;

pub mod components;
mod systems;

use systems::*;

#[derive(SystemSet, Debug, Clone, Hash, PartialEq, Eq)]
pub struct MovementSystemSet;

#[derive(SystemSet, Debug, Clone, Hash, PartialEq, Eq)]
pub struct ConfinementSystemSet;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        //? Configure the system set to run before the confinement system set
        app.configure_set(MovementSystemSet.before(ConfinementSystemSet))
            .add_startup_system(spawn_player)
            //? Explicit system ordering, all systems run in parallel by default
            // .add_system(confine_player_movement.after(player_movement))
            // .add_systems((player_movement, confine_player_movement).chain())
            .add_system(player_movement.in_set(MovementSystemSet))
            .add_system(confine_player_movement.in_set(ConfinementSystemSet))
            .add_system(enemy_hit_player)
            .add_system(player_hit_star);
    }
}
