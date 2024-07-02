mod game;
mod spawner;
use std::time::Duration;

use bevy::{
    asset::LoadState,
    core_pipeline::{bloom::BloomSettings, Skybox},
    input::touch::TouchPhase,
    prelude::*,
    render::render_resource::{TextureViewDescriptor, TextureViewDimension},
    window::{ApplicationLifetime, WindowMode},
};
use game::MainGame;
// the `bevy_main` proc_macro generates the required boilerplate for iOS and Android
#[bevy_main]
fn main() {
    let mut app = App::new();
    app.add_plugins(MainGame);
    app.run();
}
