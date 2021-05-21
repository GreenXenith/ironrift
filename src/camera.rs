use bevy::prelude::*;

pub struct Camera {}

fn camera_controller(
    mut camera_query: Query<&mut Transform, With<Camera>>,
    player_query: Query<(&crate::unit::UnitState, &Transform), (With<crate::player::Player>, Without<Camera>)>,
) {
    let (player, ptransform) = player_query.single().unwrap();
    let mut ctransform = camera_query.single_mut().unwrap();

    ctransform.rotation = player.get_look_quat();
    ctransform.translation = ptransform.translation;
}

fn camera(mut commands: Commands, mut windows: ResMut<Windows>) {
    let window = windows.get_primary_mut().unwrap();
    window.set_cursor_visibility(false);
    window.set_cursor_lock_mode(true);

    commands.spawn().insert_bundle(PerspectiveCameraBundle {
        transform: Transform::from_translation(Vec3::new(4.0, 2.0, 4.0))
            .looking_at(Vec3::new(0.0, 0.5, 0.0), Vec3::Y),
        ..Default::default()
    })
    .insert(Camera {});
}

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_startup_system(camera.system());
        app.add_system(camera_controller.system());
        app.add_plugin(crate::hud::HudPlugin);
    }
}
