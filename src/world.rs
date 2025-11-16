use avian3d::prelude::{Collider, RigidBody};
use bevy::{
    image::{ImageAddressMode, ImageLoaderSettings, ImageSampler, ImageSamplerDescriptor},
    light::NotShadowCaster,
    math::{Affine2, sampling::UniformMeshSampler},
    mesh::PlaneMeshBuilder,
    pbr::ExtendedMaterial,
    prelude::*,
};
use bevy_inspector_egui::{bevy_egui::EguiPlugin, quick::WorldInspectorPlugin};
use rand::{Rng, SeedableRng, distr::Distribution, rngs::StdRng};

use crate::{GameState, leaf_material::LeafMaterialExtension};

pub struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((EguiPlugin::default(), WorldInspectorPlugin::default()));
        app.add_systems(OnEnter(GameState::InGame), setup);
        app.add_systems(Update, leafs);
    }
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    asset_server: Res<AssetServer>,
) {
    // camera
    commands.spawn((
        Transform::from_xyz(-1.0, 0.1, 1.0).looking_at(Vec3::new(0.0, 0.0, 0.0), Vec3::Y),
        DistanceFog {
            color: Color::srgba(0.35, 0.48, 0.66, 1.0),
            directional_light_color: Color::srgba(1.0, 0.95, 0.85, 0.5),
            directional_light_exponent: 30.0,
            falloff: FogFalloff::from_visibility_colors(
                15.0, // distance in world units up to which objects retain visibility (>= 5% contrast)
                Color::srgb(0.35, 0.5, 0.66), // atmospheric extinction color (after light is lost due to absorption by atmospheric particles)
                Color::srgb(0.8, 0.844, 1.0), // atmospheric inscattering color (light gained due to scattering from the sun)
            ),
        },
    ));

    // Sun
    commands.spawn((
        DirectionalLight {
            color: Color::srgb(0.98, 0.95, 0.82),
            shadows_enabled: true,
            ..default()
        },
        Transform::from_xyz(0.0, 0.0, 0.0).looking_at(Vec3::new(-0.15, -0.05, 0.25), Vec3::Y),
    ));

    // Terrain
    let terrain_size = 200.0;
    let terrain_texture_size = 50.0;
    commands.spawn((
        Mesh3d(meshes.add(PlaneMeshBuilder::new(Dir3::Y, Vec2::splat(terrain_size)))),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color_texture: Some(asset_server.load_with_settings(
                "textures/grass.png",
                |s: &mut _| {
                    *s = ImageLoaderSettings {
                        sampler: ImageSampler::Descriptor(ImageSamplerDescriptor {
                            address_mode_u: ImageAddressMode::MirrorRepeat,
                            address_mode_v: ImageAddressMode::MirrorRepeat,
                            ..default()
                        }),
                        ..default()
                    }
                },
            )),
            uv_transform: Affine2::from_scale(Vec2::splat(terrain_size) / terrain_texture_size),
            perceptual_roughness: 1.0,
            ..default()
        })),
        Collider::cuboid(terrain_size, 0.01, terrain_size),
        RigidBody::Static,
    ));

    // Sky
    commands.spawn((
        Mesh3d(meshes.add(Cuboid::new(200.0, 100.0, 100.0))),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: Srgba::hex("888888").unwrap().into(),
            unlit: true,
            cull_mode: None,
            ..default()
        })),
        Transform::from_scale(Vec3::splat(20.0)),
        NotShadowCaster,
    ));

    // Tree
    for _ in 0..50 {
        let mut rng = StdRng::from_os_rng();
        commands.spawn((
            SceneRoot(asset_server.load("tree/tree.gltf#Scene0")),
            Transform::from_xyz(
                rng.random_range(-100.0..100.0),
                0.0,
                rng.random_range(-100.0..100.0),
            ),
        ));
    }
}

fn leafs(
    mut commands: Commands,
    mut query: Query<(&mut Transform, &GlobalTransform, &Name, &Children), Added<Name>>,
    material_mesh_query: Query<(Entity, &MeshMaterial3d<StandardMaterial>, &Mesh3d)>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut extended_materials: ResMut<
        Assets<ExtendedMaterial<StandardMaterial, LeafMaterialExtension>>,
    >,
    mut meshes: ResMut<Assets<Mesh>>,
    asset_server: Res<AssetServer>,
) {
    for (mut transform, global_transform, name, children) in query.iter_mut() {
        if !name.contains("leaf") {
            continue;
        }

        for (child_entity, child_mat, child_mesh) in material_mesh_query.iter_many(children) {
            let Some(material) = materials.get_mut(child_mat.id()) else {
                continue;
            };

            material.base_color = Color::srgba_u8(31, 38, 35, 30);
            material.alpha_mode = AlphaMode::Blend;

            let Some(mesh) = meshes.get_mut(child_mesh.id()) else {
                continue;
            };

            mesh.compute_normals();
            transform.scale = Vec3::splat(0.8);

            let sampler = UniformMeshSampler::try_new(mesh.triangles().unwrap()).unwrap();
            let rng = StdRng::seed_from_u64(4839270921);

            let leaf_mesh = PlaneMeshBuilder::from_size([2.0, 2.0].into()).build();
            let leaf_mesh = meshes.add(leaf_mesh.clone());
            let leaf_material = extended_materials.add(ExtendedMaterial {
                base: StandardMaterial {
                    base_color_texture: Some(asset_server.load("textures/leaf.png")),
                    //unlit: true,
                    cull_mode: None,
                    double_sided: true,
                    alpha_mode: AlphaMode::Blend,
                    ..default()
                },
                extension: LeafMaterialExtension {
                    fade_angle: 0.6,
                    cluster_center: global_transform.translation(),
                },
            });
            for point in sampler.sample_iter(rng).take(50) {
                /*
                leaf_mesh.compute_custom_smooth_normals(|[a, b, c], positions, normals| {
                    let center = Vec3::ZERO;
                    for i in [a, b, c] {
                        let pos = Vec3::from_array(positions[i]);
                        let normal = (pos - center).normalize_or_zero();
                        normals[i] = normal;
                    }
                });
                */

                let mut rng = StdRng::from_os_rng();

                //offset point
                let point = point + point.normalize_or_zero() * 0.5;

                commands.spawn((
                    Mesh3d(leaf_mesh.clone()),
                    MeshMaterial3d(leaf_material.clone()),
                    Transform::from_translation(point)
                        .with_scale(Vec3::splat(rng.random_range(0.1..1.5)))
                        .looking_to(
                            Dir3::new(Vec3::new(
                                rng.random_range(-1.0..=1.0),
                                rng.random_range(-1.0..=1.0),
                                rng.random_range(-1.0..=1.0),
                            ))
                            .unwrap(),
                            Dir3::Y,
                        ),
                    ChildOf(child_entity),
                    NotShadowCaster,
                ));
            }
        }
    }
}
