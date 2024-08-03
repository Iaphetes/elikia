use std::{process::exit, time::Duration};

use bevy::{input::touch::TouchPhase, prelude::*, window::AppLifecycle};
use bevy_lunex::prelude::*;

use crate::gameui::baseui::ElikiaUI;
use rand::Rng;
#[derive(Resource)]
struct MetaParameters {
    time_per_planet: Duration,
}
#[derive(Resource)]
struct PomodoroParameters {
    sprint_time: Duration,
    small_pause_time: Duration,
    long_pause_time: Duration,
    sprints: u8,
}
#[derive(Component)]
pub struct Player;
pub struct MainGame;
impl Plugin for MainGame {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    // resizable: false,
                    // mode: WindowMode::BorderlessFullscreen,
                    ..default()
                }),
                ..default()
            }),
            UiPlugin,
            ElikiaUI,
        ))
        .add_systems(
            Startup,
            (setup_scene), //, create_exploration_space.after(setup_scene)),
        )
        .add_systems(Update, (touch_camera, handle_lifetime));

        // MSAA makes some Android devices panic, this is under investigation
        // https://github.com/bevyengine/bevy/issues/8229
        #[cfg(target_os = "android")]
        app.insert_resource(Msaa::Off);

        app.insert_resource(PomodoroParameters {
            sprint_time: Duration::new(2 * 60, 0),
            small_pause_time: Duration::new(1 * 60, 0),
            long_pause_time: Duration::new(5 * 60, 0),
            sprints: 4,
        });
        app.insert_resource(MetaParameters {
            time_per_planet: Duration::new(1 * 60, 0),
        });
    }
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
fn distribute_planets(
    initial_position: &Transform,
    num_planets: u8,
    system_size: f32,
    speed: f32,
    pomodoro_interval: f32,
) -> Vec<Transform> {
    let mut planet_dist: Vec<Transform> = Vec::new();
    let mut last_pos: Transform = initial_position.clone();
    let mut rng = rand::thread_rng();
    for _p in 0..num_planets {
        let mut new_pos: Transform = Transform::from_xyz(0.0, 0.0, speed * pomodoro_interval);
        new_pos.rotate_around(
            Vec3::ZERO,
            Quat::from_rotation_y(rng.gen_range(0..360) as f32),
        );
        new_pos.translation += last_pos.translation;
        new_pos.translation = new_pos.translation.with_y(0.0);
        // info!("{:?}", new_pos);
        last_pos = new_pos;
        planet_dist.push(new_pos)
    }
    return planet_dist;
}
fn display_error_message(error_message: &str) {
    error!(error_message);
    exit(1)
}
fn create_exploration_space(
    mut commands: Commands,
    // mut meshes: ResMut<Assets<Mesh>>,
    // mut materials: ResMut<Assets<StandardMaterial>>,
    asset_server: Res<AssetServer>,
    pomodoro_parameters: Res<PomodoroParameters>,
    // meta_parameters: Res<MetaParameters>,
    player_transform: Query<&Transform, With<Player>>,
) {
    let initial_transform = player_transform
        .get_single()
        .expect("No player was instantiated");
    commands.spawn(SceneBundle {
        scene: asset_server.load("sun.gltf#Scene0"),
        ..Default::default()
    });
    let system_size: f32 = 200_000.0;
    let speed: f32 = system_size / pomodoro_parameters.sprint_time.as_secs() as f32;

    let planet_distribution: Vec<Transform> = distribute_planets(
        initial_transform,
        pomodoro_parameters.sprints,
        system_size,
        speed,
        pomodoro_parameters.sprint_time.as_secs() as f32,
    );
    for planet_transform in planet_distribution {
        commands.spawn(SceneBundle {
            scene: asset_server.load("planet.glb#Scene0"),
            transform: planet_transform.with_scale(Vec3::splat(10.0)),
            ..Default::default()
        });
    }
}

/// set up a simple 3D scene
fn setup_scene(mut commands: Commands, asset_server: Res<AssetServer>) {
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
}

// Pause audio when app goes into background and resume when it returns.
// This is handled by the OS on iOS, but not on Android.
fn handle_lifetime(
    mut lifetime_events: EventReader<AppLifecycle>,
    // music_controller: Query<&AudioSink>,
) {
    for _event in lifetime_events.read() {
        // match event {
        //     ApplicationLifetime::Suspended => music_controller.single().pause(),
        //     ApplicationLifetime::Resumed => music_controller.single().play(),
        //     ApplicationLifetime::Started => (),
        // }
    }
}
