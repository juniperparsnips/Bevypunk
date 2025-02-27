pub mod character_creator;
pub use character_creator::*;

pub mod intro;
pub use intro::*;

pub mod load_game;
pub use load_game::*;

pub mod main_menu;
pub use main_menu::*;

pub mod settings;
pub use settings::*;


// #====================#
// #=== ROUTE PLUGIN ===#

use bevy::prelude::*;

/// Plugin adding all our route logic
pub struct RoutePlugin;
impl Plugin for RoutePlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugins(CharacterCreatorRoutePlugin)
            .add_plugins(IntroRoutePlugin)
            .add_plugins(LoadGameRoutePlugin)
            .add_plugins(MainMenuRoutePlugin)
            .add_plugins(SettingsRoutePlugin);
    }
}