use bevy::{
    input::{keyboard::KeyCode, mouse::MouseMotion},
    math,
    prelude::*,
};

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
            velocity: Vec3::zero(),
			sensitivity: 10.0,
			speed: 5.0,
        }
    }
}

fn camera_controller(
    time: Res<Time>,
    keys: Res<Input<KeyCode>>,

    motion: Res<Events<MouseMotion>>,
    mut reader: Local<EventReader<MouseMotion>>,

    mut query: Query<(&mut CameraState, &mut Transform)>,
) {
    let delta_s = time.delta_seconds();

    let mut delta_m: Vec2 = Vec2::zero();
    for event in reader.iter(&motion) {
        delta_m += event.delta;
    }

    for (mut state, mut transform) in query.iter_mut() {
        if !delta_m.is_nan() {
            state.yaw -= delta_m.x * state.sensitivity * delta_s;
            state.pitch += delta_m.y * state.sensitivity * delta_s;

            state.pitch = math::clamp(state.pitch, -89.9, 89.9);

            transform.rotation = Quat::from_axis_angle(Vec3::unit_y(), state.yaw.to_radians())
                        * Quat::from_axis_angle(-Vec3::unit_x(), state.pitch.to_radians());
        }

        state.velocity = Vec3::zero();

        let forward = transform.rotation.mul_vec3(Vec3::unit_z()).normalize() * Vec3::new(1.0, 0.0, 1.0);
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

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
	fn build(&self, app: &mut AppBuilder) {
		app.add_system(camera_controller.system());
	}
}
