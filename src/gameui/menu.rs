use bevy::{
    asset::LoadState,
    core_pipeline::{bloom::BloomSettings, tonemapping::Tonemapping, Skybox},
    prelude::*,
    render::render_resource::*,
};
use bevy_lunex::prelude::*;

use crate::game::Player;

// pub struct Menu;
// impl Plugin for Menu {
//     fn build(&self, app: &mut App) {
//         app.add_systems(Startup, setup_menu);
//     }
// }
// fn setup_menu(mut commands: Commands, asset_server: Res<AssetServer>) {
//     commands
//             // Here we will spawn our UI as children
//             ui.spawn((
//                 // #=== UI DEFINITION ===#

//                 // This specifies the name and hierarchy of the node
//                 UiLink::<MainUi>::path("Camera/Menu"),
//                 UiLayout::window_full()
//                     .pos(Ab(20.0))
//                     .size(Rl(100.0) - Ab(20.0))
//                     .pack::<Base>(),
//             ));
//             ui.spawn((
//                 UiLink::<MainUi>::path("Camera/Menu/Button"),
//                 // Here you can define the layout using the provided units (per state like Base, Hover, Selected, etc.)
//                 UiLayout::window()
//                     .pos(Rl((50.0, 50.0)))
//                     .size(Rl((10.0, 5.0)))
//                     // .size((Ab(1920.0), Ab(1080.0)))
//                     .pack::<Base>(),
//                 // #=== CUSTOMIZATION ===#

//                 // Give it a background image
//                 UiImage2dBundle {
//                     texture: asset_server.load("ui/button.png"),
//                     ..default()
//                 },
//                 // Make the background image resizable
//                 ImageScaleMode::Sliced(TextureSlicer {
//                     border: BorderRect::square(64.0),
//                     ..default()
//                 }),
//                 // // This is required to control our hover animation
//                 // UiAnimator::<Hover>::new()
//                 //     .forward_speed(5.0)
//                 //     .backward_speed(1.0),
//                 // // This will set the base color to red
//                 // UiColor::<Base>::new(RED.into()),
//                 // // This will set hover color to yellow
//                 // UiColor::<Hover>::new(YELLOW.into()),
//                 // // #=== INTERACTIVITY ===#

//                 // // This is required for hit detection (make it clickable)
//                 // PickableBundle::default(),
//                 // // This will change cursor icon on mouse hover
//                 // OnHoverSetCursor::new(CursorIcon::Pointer),
//                 // // If we click on this, it will emmit UiClick event we can listen to
//                 // UiClickEmitter::SELF,
//             ));
//         });
// }

fn launch_game(asset_server: Res<AssetServer>, mut commands: Commands) {}
