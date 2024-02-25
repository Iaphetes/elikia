use std::time::Duration;

use bevy::{
    asset::LoadState,
    core_pipeline::{bloom::BloomSettings, Skybox},
    input::touch::TouchPhase,
    prelude::*,
    render::render_resource::{TextureViewDescriptor, TextureViewDimension},
    window::{ApplicationLifetime, WindowMode},
};



#[derive(Resource)]
struct Cubemap {
    is_loaded: bool,
    index: usize,
    image_handle: Handle<Image>,
}


#[derive(Resource)]
struct MetaParameters {
    time_per_planet: Duration
}
#[derive(Resource)]
struct PomodoroParameters {
    sprint_time: Duration,
    small_pause_time: Duration,
    long_pause_time: Duration,
    sprints: u8,
}
// the `bevy_main` proc_macro generates the required boilerplate for iOS and Android
#[bevy_main]
fn main() {
    let mut app = App::new();
    app.add_plugins(DefaultPlugins.set(WindowPlugin {
        primary_window: Some(Window {
            resizable: false,
            mode: WindowMode::BorderlessFullscreen,
            ..default()
        }),
        ..default()
    }))
    .add_systems(Startup, (setup_scene, create_exploration_space))
    .add_systems(
        Update,
        (asset_loaded, touch_camera, button_handler, handle_lifetime),
    );

    // MSAA makes some Android devices panic, this is under investigation
    // https://github.com/bevyengine/bevy/issues/8229
    #[cfg(target_os = "android")]
    app.insert_resource(Msaa::Off);

    app.insert_resource(PomodoroParameters {
        sprint_time: Duration::new(2 * 60, 0),
        small_pause_time: Duration::new(1* 60, 0),
        long_pause_time: Duration::new(5 * 60, 0),
        sprints: 4
    });
    app.insert_resource(MetaParameters{time_per_planet:Duration::new(1 * 60, 0)});
    app.run();
}

fn touch_camera(
    windows: Query<&Window>,
    mut touches: EventReader<TouchInput>,
    mut camera: Query<&mut Transform, With<Camera3d>>,
    mut last_position: Local<Option<Vec2>>,
) {
    let window = windows.single();

    for touch in touches.read() {
        if touch.phase == TouchPhase::Started {
            *last_position = None;
        }
        if let Some(last_position) = *last_position {
            let mut transform = camera.single_mut();
            *transform = Transform::from_xyz(
                transform.translation.x
                    + (touch.position.x - last_position.x) / window.width() * 5.0,
                transform.translation.y,
                transform.translation.z
                    + (touch.position.y - last_position.y) / window.height() * 5.0,
            )
            .looking_at(Vec3::ZERO, Vec3::Y);
        }
        *last_position = Some(touch.position);
    }
}

fn create_exploration_space(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    asset_server: Res<AssetServer>,
    pomodoro_parametesr: Res<PomodoroParameters>,
    meta_parameters: Res<MetaParameters>
){
    commands.spawn(SceneBundle {
        scene: asset_server.load("sun.gltf#Scene0"),
        ..Default::default()
    });
    let time_per_planet: u8 =(pomodoro_parametesr.sprint_time.as_secs() as f64 / meta_parameters.time_per_planet.as_secs() as f64).round() as u8; 
    for _ in 0..time_per_planet - 1{
        
        commands.spawn(SceneBundle {
            scene: asset_server.load("planet.glb#Scene0"),
            transform: Transform::from_xyz(16_772.0, 11_000.0, 0.0),
            ..Default::default()
        });
    }
}

/// set up a simple 3D scene
fn setup_scene(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    asset_server: Res<AssetServer>,
) {
    // commands.spawn(SceneBundle {
    //     scene: asset_server.load("sun.gltf#Scene0"),
    //     transform: Transform::from_xyz(150_000_010_000.0, 0.0, 0.0)
    //         .with_scale(Vec3::splat(100000.0)),
    //     ..Default::default()
    // });
    // commands.spawn(SceneBundle {
    //     scene: asset_server.load("planet.glb#Scene0"),
    //     transform: Transform::from_xyz(16_772.0, 11_000.0, 0.0),
    //     ..Default::default()
    // });
    // light
    commands.spawn(PointLightBundle {
        transform: Transform::from_xyz(4.0, 8.0, 4.0),
        point_light: PointLight {
            intensity: 5000.0,
            // Shadows makes some Android devices segfault, this is under investigation
            // https://github.com/bevyengine/bevy/issues/8214
            #[cfg(not(target_os = "android"))]
            shadows_enabled: true,
            ..default()
        },
        ..default()
    });
    let skybox_handle: Handle<Image> = asset_server.load("textures/skybox/stacked.png");
    // camera
    commands.spawn((
        Camera3dBundle {
            transform: Transform::from_xyz(150_000.0, 0.0, 0.0).looking_at(-Vec3::X, Vec3::Z),

            camera: Camera {
                hdr: true,

                ..default()
            },
            ..default()
        },
        BloomSettings::default(),
        Skybox(skybox_handle.clone()),
    ));
    commands.insert_resource(Cubemap {
        is_loaded: false,
        index: 0,
        image_handle: skybox_handle,
    });
    // Test ui
    commands
        .spawn(ButtonBundle {
            style: Style {
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                position_type: PositionType::Absolute,
                left: Val::Px(50.0),
                right: Val::Px(50.0),
                bottom: Val::Px(50.0),
                ..default()
            },
            ..default()
        })
        .with_children(|b| {
            b.spawn(TextBundle::from_section(
                "Test Button",
                TextStyle {
                    font_size: 30.0,
                    color: Color::BLACK,
                    ..default()
                },
            ));
        });
}
fn asset_loaded(
    asset_server: Res<AssetServer>,
    mut images: ResMut<Assets<Image>>,
    mut cubemap: ResMut<Cubemap>,
    mut skyboxes: Query<&mut Skybox>,
) {
    if !cubemap.is_loaded
        && asset_server.get_load_state(cubemap.image_handle.clone_weak()) == Some(LoadState::Loaded)
    {
        let image = images.get_mut(&cubemap.image_handle).unwrap();
        // NOTE: PNGs do not have any metadata that could indicate they contain a cubemap texture,
        // so they appear as one texture. The following code reconfigures the texture as necessary.
        if image.texture_descriptor.array_layer_count() == 1 {
            image.reinterpret_stacked_2d_as_array(
                image.texture_descriptor.size.height / image.texture_descriptor.size.width,
            );
            image.texture_view_descriptor = Some(TextureViewDescriptor {
                dimension: Some(TextureViewDimension::Cube),
                ..default()
            });
        }

        for mut skybox in &mut skyboxes {
            skybox.0 = cubemap.image_handle.clone();
        }

        cubemap.is_loaded = true;
    }
}
fn button_handler(
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor),
        (Changed<Interaction>, With<Button>),
    >,
) {
    for (interaction, mut color) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                *color = Color::BLUE.into();
            }
            Interaction::Hovered => {
                *color = Color::GRAY.into();
            }
            Interaction::None => {
                *color = Color::WHITE.into();
            }
        }
    }
}

fn setup_music(asset_server: Res<AssetServer>, mut commands: Commands) {
    commands.spawn(AudioBundle {
        source: asset_server.load("sounds/Windless Slopes.ogg"),
        settings: PlaybackSettings::LOOP,
    });
}

// Pause audio when app goes into background and resume when it returns.
// This is handled by the OS on iOS, but not on Android.
fn handle_lifetime(
    mut lifetime_events: EventReader<ApplicationLifetime>,
    music_controller: Query<&AudioSink>,
) {
    for event in lifetime_events.read() {
        // match event {
        //     ApplicationLifetime::Suspended => music_controller.single().pause(),
        //     ApplicationLifetime::Resumed => music_controller.single().play(),
        //     ApplicationLifetime::Started => (),
        // }
    }
}
