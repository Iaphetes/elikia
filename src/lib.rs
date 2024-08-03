mod game;
mod gameui;
use bevy::prelude::*;
use game::MainGame;
// the `bevy_main` proc_macro generates the required boilerplate for iOS and Android
#[bevy_main]
fn main() {
    let mut app = App::new();
    app.add_plugins(MainGame);
    app.run();
}
