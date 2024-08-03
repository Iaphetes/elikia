use crate::game::Player;
use bevy::{
    asset::LoadState,
    core_pipeline::{bloom::BloomSettings, tonemapping::Tonemapping, Skybox},
    prelude::*,
    render::render_resource::*,
};
use bevy_lunex::prelude::*;
pub struct ElikiaUI;
impl Plugin for ElikiaUI {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, initialize_ui)
            .add_systems(Update, asset_loaded);
    }
}

#[derive(Resource)]
struct Cubemap {
    is_loaded: bool,
    index: usize,
    image_handle: Handle<Image>,
}
fn initialize_ui(mut commands: Commands, asset_server: Res<AssetServer>) {
    // Create a texture resource that our 3D camera will render to
    let size = Extent3d {
        width: 2560,
        height: 1440,
        ..default()
    };

    // Create the texture
    let mut image = Image {
        texture_descriptor: TextureDescriptor {
            label: None,
            size,
            dimension: TextureDimension::D2,
            format: TextureFormat::Bgra8UnormSrgb,
            mip_level_count: 1,
            sample_count: 1,
            usage: TextureUsages::TEXTURE_BINDING
                | TextureUsages::COPY_DST
                | TextureUsages::RENDER_ATTACHMENT,
            view_formats: &[],
        },
        ..default()
    };

    // Initiate the image
    image.resize(size);

    // Add our texture to asset server and get a handle
    let render_image = asset_server.add(image);
    let skybox_handle: Handle<Image> = asset_server.load("textures/skybox/stacked.png");
    // camera
    commands.spawn((
        // Camera3dBundle {
        //     transform: Transform::from_xyz(150_000.0, 0.0, 0.0).looking_at(-Vec3::X, Vec3::Z),

        //     camera: Camera {
        //         hdr: true,

        //         ..default()
        //     },
        //     tonemapping: Tonemapping::TonyMcMapface,
        //     ..default()
        // },
        // BloomSettings::default(),
        // Skybox {
        //     image: skybox_handle.clone(),
        //     brightness: 1000.0,
        // },
        Camera3dBundle {
            transform: Transform::from_xyz(0.0, 600_000.0, 0.0).looking_at(-Vec3::Z, Vec3::Z),

            camera: Camera {
                hdr: true,
                order: -1,
                target: render_image.clone().into(),
                clear_color: ClearColorConfig::Custom(Color::srgba(0.0, 0.0, 0.0, 0.0)),
                ..default()
            },
            tonemapping: Tonemapping::TonyMcMapface,
            ..default()
        },
        BloomSettings::default(),
        Skybox {
            image: skybox_handle.clone(),
            brightness: 1000.0,
        },
        MainUi,
        Player,
    ));
    commands.insert_resource(Cubemap {
        is_loaded: false,
        index: 0,
        image_handle: skybox_handle,
    });
    commands.spawn((
        MainUi,
        Camera2dBundle {
            transform: Transform::from_xyz(0.0, 0.0, 1000.0),
            ..default()
        },
    ));
    commands
        .spawn((
            // This makes the UI entity able to receive camera data
            MovableByCamera,
            // This is our UI system
            UiTreeBundle::<MainUi>::from(UiTree::new2d("Hello UI!")),
        ))
        .with_children(|ui| {
            ui.spawn((
                // root.add("Camera3d"),
                UiLink::<MainUi>::path("Camera"),
                UiLayout::solid()
                    .size((2560.0, 1440.0))
                    .scaling(Scaling::Fill)
                    .pack::<Base>(),
                UiImage2dBundle::from(render_image),
                PickingPortal, // You can add this component to send picking events through the viewport.
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
        && asset_server.get_load_state(&cubemap.image_handle) == Some(LoadState::Loaded)
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
            skybox.image = cubemap.image_handle.clone();
        }

        cubemap.is_loaded = true;
    }
}
