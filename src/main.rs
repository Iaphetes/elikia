mod game;
mod spawner;

use bevy::prelude::*;
use game::MainGame;
fn main() {
    let mut app = App::new();
    app.add_plugins((MainGame, spawner::Spawner));
    app.run();
}
