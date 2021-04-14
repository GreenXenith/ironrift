use bevy::prelude::*;

fn map_loader(
    mut commands: Commands,
    mut materials: ResMut<Assets<StandardMaterial>>,
    assets: Res<AssetServer>,
) {
    let choice = 1;
    let maps: [&str; 2] = ["monke", "testmap"];

    let map: Handle<Mesh> = assets.load(format!("models/maps/{}.glb#Mesh0/Primitive0", maps[choice]).as_str());
    let mat = materials.add(Color::rgb(0.6, 0.9, 0.6).into());

    commands.spawn().insert_bundle(PbrBundle {
        mesh: map,
        material: mat,
        transform: Transform::from_translation(Vec3::new(0.0, 0.5, 0.0)),
        ..Default::default()
    });

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
        app.add_startup_system(map_loader.system());
	}
}
