pub mod events;
mod game;
mod main_menu;
mod systems;

use systems::*;

use bevy::prelude::*;

fn main() {
    App::new()
        //* Bevy's default plugins
        .add_plugins(DefaultPlugins)
        .add_state::<AppState>()
        //* My plugins
        .add_plugin(game::GamePlugin)
        .add_plugin(main_menu::MainMenuPlugin)
        //* Startup systems
        .add_startup_system(spawn_camera)
        // *Systems
        .add_system(transition_to_game_state)
        .add_system(transition_to_main_menu_state)
        .add_system(exit_game)
        .add_system(handle_game_over)
        .run();
}

#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
pub enum AppState {
    #[default]
    MainMenu,
    Game,
    GameOver,
}
