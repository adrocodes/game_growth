mod actions;
mod audio;
mod bounds;
mod building;
mod global_state;
mod loading;
mod menu;
mod mouse_position;
mod player;
mod tiles;
mod ui;
mod world_gen;

use crate::audio::InternalAudioPlugin;
use crate::global_state::GlobalState;
use crate::loading::LoadingPlugin;
use crate::menu::MenuPlugin;
use crate::world_gen::WorldGenPlugin;

use bevy::app::App;
#[cfg(debug_assertions)]
use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};
use bevy::prelude::*;
use building::BuildingPlugin;
use mouse_position::MousePositionPlugin;

// This example game uses States to separate logic
// See https://bevy-cheatbook.github.io/programming/states.html
// Or https://github.com/bevyengine/bevy/blob/main/examples/ecs/state.rs
#[derive(Clone, Eq, PartialEq, Debug, Hash)]
enum GameState {
    // During the loading State the LoadingPlugin will load our assets
    Loading,
    // During this State the actual game logic is executed
    Playing,
    // Here the menu is drawn and waiting for player interaction
    Menu,
}

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(GlobalState::default())
            .add_state(GameState::Loading)
            .add_plugin(LoadingPlugin)
            .add_plugin(MenuPlugin)
            .add_plugin(MousePositionPlugin)
            .add_plugin(InternalAudioPlugin)
            .add_plugin(WorldGenPlugin)
            .add_plugin(ui::GuiPlugin)
            .add_plugin(BuildingPlugin);

        #[cfg(debug_assertions)]
        {
            app.add_plugin(FrameTimeDiagnosticsPlugin::default())
                .add_plugin(LogDiagnosticsPlugin::default());
        }
    }
}
