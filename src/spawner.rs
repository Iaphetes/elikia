use bevy::{asset::AssetIndex, gltf::Gltf, prelude::*, reflect::Enum};

pub struct Spawner;

impl Plugin for Spawner {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, load_gltf)
            .add_systems(Update, (spawn_gltf_objects, extend_material));
    }
}

pub fn load_gltf(mut commands: Commands, assets: Res<AssetServer>) {
    let my_gltf = assets.load("material_test.glb");

    commands.spawn(SpawnAsset { handle: my_gltf });
}
#[derive(Component)]
pub struct EntityWrapper(Entity);

#[derive(Component)]
pub struct SpawnAsset {
    pub handle: Handle<Gltf>,
}

#[derive(Component)]
pub struct AssetExtension {
    asset_index: AssetId<StandardMaterial>,
    emissive_color: Color,
}

pub fn spawn_gltf_objects(
    mut commands: Commands,
    mut spawn_assets: Query<(Entity, &mut SpawnAsset)>,
    assets_gltf: Res<Assets<Gltf>>,
    mut materials: Query<&mut Handle<StandardMaterial>>,
) {
    // This flag is used to because this system has to be run until the asset is loaded.
    // If there's a better way of going about this I am unaware of it.
    // for material in &mut materials {
    //     info!("{:#?}", material);
    // }
    for (entity, mut spawn_asset) in &mut spawn_assets {
        if let Some(gltf) = assets_gltf.get(&spawn_asset.handle) {
            commands.spawn(SceneBundle {
                scene: gltf.scenes[0].clone(),

                transform: Transform::from_xyz(149_980.0, 10.0, 0.0),
                ..Default::default()
            });

            let source = gltf.source.as_ref().unwrap();
            gltf.source.as_ref();
            // info!("{:#?}", source.meshes());
            // info!("{:#?}", source.materials());
            for mesh in source.meshes() {
                for primitive in mesh.primitives() {
                    // info!("{:#?}", primitive);
                }
            }
            for material in &gltf.named_materials {
                info!("{:#?}", material);
                for raw_material in source.materials() {
                    if let Some(name) = raw_material.name() {
                        if material.0 == name {
                            if let Some(emissive_strength) = raw_material.emissive_strength() {
                                info!("{:#?}", emissive_strength);
                                let mut emissive_color: [f32; 3] = raw_material.emissive_factor();
                                emissive_color[0] *= emissive_strength;
                                emissive_color[1] *= emissive_strength;
                                emissive_color[2] *= emissive_strength;
                                // info!("\n\n\n{:#?}\n\n\n", raw_material);
                                commands.spawn(AssetExtension {
                                    asset_index: material.1.id(),
                                    emissive_color: Color::rgb_from_array(emissive_color),
                                });
                            }
                        }
                    }
                    // info!("Raw material: {:#?}", raw_material.name());
                }
            }
        }
        if let Some(gltf) = assets_gltf.get(&spawn_asset.handle) {
            info!("spawn");
            // spawn the first scene in the file

            let e = commands.spawn(
                (SceneBundle {
                    scene: gltf.scenes[0].clone(),
                    ..Default::default()
                }),
            );
            // e.insert(AssetExtension { emissive_strength });
        }

        commands.entity(entity).despawn();
    }
}

pub fn extend_material(
    mut commands: Commands,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut extended_assets: Query<(Entity, &AssetExtension)>,
) {
    // for (scene_handle, asset_extension) in &extended_assets {
    //     info!("{:?}", asset_extension.emissive_strength);
    // }
    // for material in &mut materials {
    //     // info!("Spawned Material:{:#?}", material);
    // }
    for (entity, asset_extension) in &extended_assets {
        if let Some(material) = materials.get_mut(asset_extension.asset_index) {
            material.emissive = asset_extension.emissive_color;
        }
        commands.entity(entity).despawn_recursive();
        // info!("{:?}", materials.get(asset_extension.asset_index));
    }
}
