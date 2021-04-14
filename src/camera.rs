use bevy::prelude::*;
use bevy::input::{keyboard::KeyCode, mouse::MouseMotion};
pub struct CameraState {
    pub pitch: f32,
    pub yaw: f32,
	pub velocity: Vec3,
	pub speed: f32,
	pub sensitivity: f32,
}

impl Default for CameraState {
    fn default() -> Self {
        Self {
            pitch: 0.0,
            yaw: 0.0,
            velocity: Vec3::ZERO,
			sensitivity: 10.0,
			speed: 5.0,
        }
    }
}

fn camera_controller(
    time: Res<Time>,
    keys: Res<Input<KeyCode>>,

    mut reader: EventReader<MouseMotion>,

    mut query: Query<(&mut CameraState, &mut Transform)>,
) {
    let delta_s = time.delta_seconds();

    let mut delta_m: Vec2 = Vec2::ZERO;
    for event in reader.iter() {
        delta_m += event.delta;
    }

    for (mut state, mut transform) in query.iter_mut() {
        if !delta_m.is_nan() {
            state.yaw -= delta_m.x * state.sensitivity * delta_s;
            state.pitch += delta_m.y * state.sensitivity * delta_s;

            state.pitch = state.pitch.clamp(-89.9, 89.9);

            transform.rotation = Quat::from_axis_angle(Vec3::Y, state.yaw.to_radians())
                        * Quat::from_axis_angle(-Vec3::X, state.pitch.to_radians());
        }

        state.velocity = Vec3::ZERO;

        let forward = transform.rotation.mul_vec3(Vec3::Z).normalize() * Vec3::new(1.0, 0.0, 1.0);
        let strafe = Quat::from_rotation_y(90.0f32.to_radians()).mul_vec3(forward).normalize();

        if keys.pressed(KeyCode::W) {
            state.velocity -= forward;
        }

        if keys.pressed(KeyCode::S) {
            state.velocity += forward;
        }

        if keys.pressed(KeyCode::A) {
            state.velocity -= strafe;
        }

        if keys.pressed(KeyCode::D) {
            state.velocity += strafe;
        }

        if keys.pressed(KeyCode::Space) {
            state.velocity.y += 1.0;
        }

        if keys.pressed(KeyCode::LShift) {
            state.velocity.y -= 1.0;
        }

		let speed = state.speed;
        state.velocity *= speed * delta_s;
        transform.translation += state.velocity;
    }
}

fn camera(mut commands: Commands, mut windows: ResMut<Windows>) {
    let window = windows.get_primary_mut().unwrap();
    window.set_cursor_visibility(false);
    window.set_cursor_lock_mode(true);

    let mut state = CameraState::default();
    state.yaw = 45.0;
    state.pitch = 15.0;

    commands.spawn_bundle(PerspectiveCameraBundle {
        transform: Transform::from_translation(Vec3::new(4.0, 2.0, 4.0))
            .looking_at(Vec3::new(0.0, 0.5, 0.0), Vec3::Y),
        ..Default::default()
    }).insert(state);
}

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
	fn build(&self, app: &mut AppBuilder) {
        app.add_startup_system(camera.system());
		app.add_system(camera_controller.system());
	}
}
