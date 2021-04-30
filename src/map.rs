use bevy::prelude::*;
use bevy::asset::LoadState;
use bevy_rapier3d::physics::RapierPhysicsPlugin;
use bevy_rapier3d::rapier::dynamics::RigidBodyBuilder;
use bevy_rapier3d::rapier::geometry::ColliderBuilder;
// use bevy_rapier3d::render::RapierRenderPlugin;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum AppState {
    Loading,
    Loaded,
}

#[derive(Default)]
struct MapHandles {
    handles: Vec<HandleUntyped>,
}

fn load_maps(mut map_handles: ResMut<MapHandles>, assets: Res<AssetServer>) {
    map_handles.handles = assets.load_folder("models/maps").unwrap();
}

fn check_maps(
    mut state: ResMut<State<AppState>>,
    map_handles: ResMut<MapHandles>,
    assets: Res<AssetServer>,
) {
    if let LoadState::Loaded = assets.get_group_load_state(map_handles.handles.iter().map(|handle| handle.id)) {
        state.set(AppState::Loaded).unwrap();
    }
}

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
    // map_handles: Res<MapHandles>,
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
    .insert(mesh_collider(meshes.get("models/maps/testmap.glb#Mesh0/Primitive0").unwrap()).user_data(1));

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
        app.init_resource::<MapHandles>();
        app.add_state(AppState::Loading);
        app.add_system_set(SystemSet::on_enter(AppState::Loading).with_system(load_maps.system()));
        app.add_system_set(SystemSet::on_update(AppState::Loading).with_system(check_maps.system()));
        app.add_system_set(SystemSet::on_enter(AppState::Loaded).with_system(initialize_map.system()));
        app.add_plugin(RapierPhysicsPlugin);
        // app.add_plugin(RapierRenderPlugin);
	}
}
