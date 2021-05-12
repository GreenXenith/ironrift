use bevy::prelude::*;
use bevy::input::mouse::MouseMotion;

pub struct CameraState {
    pub pitch: f32,
    pub yaw: f32,
}

impl Default for CameraState {
    fn default() -> Self {
        Self {
            pitch: 0.0,
            yaw: 0.0,
        }
    }
}

fn camera_controller(
    mut reader: EventReader<MouseMotion>,

    mut camera_query: Query<(&mut CameraState, &mut Transform), With<CameraState>>,
    player_query: Query<(&crate::player::Player, &Transform), Without<CameraState>>,
) {
    let mut delta_m: Vec2 = Vec2::ZERO;
    for event in reader.iter() {
        delta_m += event.delta;
    }

    let (player, ptransform) = player_query.single().unwrap();
    let (mut state, mut transform) = camera_query.single_mut().unwrap();

    if !delta_m.is_nan() {
        state.yaw = player.yaw;
        state.pitch = player.pitch;

        transform.rotation = Quat::from_axis_angle(Vec3::Y, state.yaw.to_radians())
                    * Quat::from_axis_angle(-Vec3::X, state.pitch.to_radians());
    }

    transform.translation = ptransform.translation;
}

fn camera(mut commands: Commands, mut windows: ResMut<Windows>) {
    let window = windows.get_primary_mut().unwrap();
    window.set_cursor_visibility(false);
    window.set_cursor_lock_mode(true);

    let mut state = CameraState::default();
    state.yaw = 45.0;
    state.pitch = 15.0;

    commands.spawn().insert_bundle(PerspectiveCameraBundle {
        transform: Transform::from_translation(Vec3::new(4.0, 2.0, 4.0))
            .looking_at(Vec3::new(0.0, 0.5, 0.0), Vec3::Y),
        ..Default::default()
    })
    .insert(state);
}

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_startup_system(camera.system());
        app.add_system(camera_controller.system());
        app.add_plugin(crate::hud::HudPlugin);
    }
}
