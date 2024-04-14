use bevy::{log::info, prelude::*};

use crate::{
    audio::InternalAudioPlugin, core::CorePlugin, graphics::GraphicsPlugin, input::InputPlugin,
    ui::UiPlugin,
};

mod audio;
mod core;
mod graphics;
mod input;
mod ui;

// Use of a mod or pub mod is not actually necessary.
pub mod built_info {
    // The file has been placed there by the build script.
    include!(concat!(env!("OUT_DIR"), "/built.rs"));
}

#[derive(States, Debug, Default, Clone, PartialEq, Eq, Hash)]
pub enum AppState {
    #[default]
    Menu,
    Game,
}

fn main() {
    let mut app = App::new();
    app.init_state::<AppState>()
        .add_plugins((DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: format!("Necromatcher v{}", built_info::PKG_VERSION),
                resolution: (1280., 720.).into(),
                ..default()
            }),
            ..default()
        }),))
        .add_plugins((
            CorePlugin,
            InputPlugin,
            GraphicsPlugin,
            UiPlugin,
            InternalAudioPlugin,
        ));

    info!(
        "Starting Necromatcher client application - v{} - SHA: {}",
        built_info::PKG_VERSION,
        built_info::GIT_COMMIT_HASH_SHORT.unwrap_or("unknown")
    );
    app.run();
}
