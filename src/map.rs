use bevy::prelude::*;
use bevy_rapier3d::physics::RapierPhysicsPlugin;
use bevy_rapier3d::rapier::dynamics::RigidBodyBuilder;
use bevy_rapier3d::rapier::geometry::ColliderBuilder;
// use bevy_rapier3d::render::RapierRenderPlugin;

fn mesh_collider(mesh: &Mesh) -> ColliderBuilder {
    use bevy_rapier3d::na::Point3;

    let mut vertices: Vec<Point3<f32>> = Vec::new();
    let mut indices: Vec<[u32; 3]> = Vec::new();

    match mesh.attribute(Mesh::ATTRIBUTE_POSITION).unwrap() {
        bevy::render::mesh::VertexAttributeValues::Float3(attr) => {
            for pos in attr.iter() {
                vertices.push(Point3::new(pos[0], pos[1], pos[2]));
            }
        }
        _ => {}
    }

    match mesh.indices().unwrap() {
        bevy::render::mesh::Indices::U32(ind) => {
            for i in 0 .. (ind.len() / 3) {
                indices.push([ind[i * 3], ind[i * 3 + 1], ind[i * 3 + 2]]);
            }
        }
        _ => {}
    }

    // use bevy_rapier3d::rapier::parry;
    // let (vertices, indices) = parry::shape::Cuboid::new(parry::math::Vector::new(0.5, 0.5, 0.5)).to_trimesh();

    return ColliderBuilder::trimesh(vertices, indices);
}

fn initialize_map(
    mut commands: Commands,
    mut materials: ResMut<Assets<StandardMaterial>>,

    assets: Res<AssetServer>,
    meshes: Res<Assets<Mesh>>,
) {
    // Load map
    let choice = 1;
    let maps: [&str; 2] = ["monke", "testmap"];

    commands.spawn().insert_bundle(PbrBundle {
        mesh: assets.get_handle(format!("models/maps/{}.glb#Mesh0/Primitive0", maps[choice]).as_str()),
        material: materials.add(Color::rgb(0.6, 0.9, 0.6).into()),
        transform: Transform::from_translation(Vec3::new(0.0, 0.0, 0.0)),
        ..Default::default()
    })
    .insert(RigidBodyBuilder::new_static().translation(0.0, 0.0, 0.0))
    .insert(mesh_collider(meshes.get("models/maps/testmap.glb#Mesh0/Primitive0").unwrap()).user_data(crate::ObjectType::Terrain as u128));

    // Light
    commands.spawn().insert_bundle(LightBundle {
        light: Light {
            intensity: 100000.0,
            range: 1000.0,
            ..Default::default()
        },
        transform: Transform::from_translation(Vec3::new(100.0, 100.0, 100.0)),
        ..Default::default()
    });
}

pub struct MapPlugin;

impl Plugin for MapPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_system_set(SystemSet::on_enter(crate::AppState::Loaded).with_system(initialize_map.system()));
        app.add_plugin(RapierPhysicsPlugin);
        // app.add_plugin(RapierRenderPlugin);
    }
}
