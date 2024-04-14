use bevy::{log::info, prelude::App, DefaultPlugins};

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

fn main() {
    let mut app = App::new();
    app.add_plugins(DefaultPlugins).add_plugins((
        CorePlugin,
        InputPlugin,
        GraphicsPlugin,
        UiPlugin,
        InternalAudioPlugin,
    ));

    info!(
        "Starting client application - v{} - SHA: {}",
        built_info::PKG_VERSION,
        built_info::GIT_COMMIT_HASH_SHORT.unwrap_or("unknown")
    );
    app.run();
}
