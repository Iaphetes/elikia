pub mod game;
pub mod gameui;

use bevy::prelude::*;
use game::MainGame;
fn main() {
    let mut app = App::new();
    app.add_plugins(MainGame);
    app.run();
}
