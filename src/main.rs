use bevy::{
    input::keyboard::KeyCode,
    prelude::*,
};

mod camera;
use camera::{CameraState, CameraPlugin};

fn main() {
    App::build()
        .add_resource(Msaa { samples: 4 })
        .add_resource(WindowDescriptor {
            title: "Ironrift".to_string(),
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup.system())
        .add_system(quit.system())
        .add_plugin(CameraPlugin)
        .run();
}

fn quit (keys: Res<Input<KeyCode>>, mut exit: ResMut<Events<bevy::app::AppExit>>) {
    if keys.pressed(KeyCode::Escape) {
        return exit.send(bevy::app::AppExit);
    }
}

fn setup(
    commands: &mut Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut windows: ResMut<Windows>,
) {
    let window = windows.get_primary_mut().unwrap();
    window.set_cursor_visibility(false);
    window.set_cursor_lock_mode(true);

    let mut state = CameraState::default();
    state.yaw = 45.0;
    state.pitch = 15.0;

    commands
        // Cube
        .spawn(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
            material: materials.add(Color::rgb(0.6, 0.9, 0.6).into()),
            transform: Transform::from_translation(Vec3::new(0.0, 0.5, 0.0)),
            ..Default::default()
        })
        // Light
        .spawn(LightBundle {
            transform: Transform::from_translation(Vec3::new(4.0, 8.0, 12.0)),
            ..Default::default()
        })
        // Camera
        .spawn(Camera3dBundle {
            transform: Transform::from_translation(Vec3::new(4.0, 2.0, 4.0))
                .looking_at(Vec3::new(0.0, 0.5, 0.0), Vec3::unit_y()),
            ..Default::default()
        })
        .with(state);
}
