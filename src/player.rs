use bevy::prelude::*;
use bevy::input::keyboard::KeyCode;
use bevy_rapier3d::rapier::na;
use crate::unit;

pub struct Player {
    pub sensitivity: f32,
    pub speed: f32,
}

impl Default for Player {
    fn default() -> Self {
        Self {
            sensitivity: 10.0,
            speed: 5.0,
        }
    }
}

fn player_controller(
    time: Res<Time>,
    mut mousemotion: EventReader<bevy::input::mouse::MouseMotion>,
    mousebutton: Res<Input<bevy::input::mouse::MouseButton>>,
    keypress: Res<Input<KeyCode>>,

    mut query: Query<(&Player, &mut unit::UnitState)>,
) {
    let (player, mut unit) = query.single_mut().unwrap();

    let delta_s = time.delta_seconds();
    let mut delta_m = Vec2::ZERO;

    for event in mousemotion.iter() {
        delta_m += event.delta;
    }

    if !delta_m.is_nan() {
        unit.pitch = (unit.pitch - (delta_m.y * player.sensitivity * delta_s).to_radians()).clamp(-std::f32::consts::FRAC_PI_2, std::f32::consts::FRAC_PI_2);
        unit.yaw += -(delta_m.x * player.sensitivity * delta_s).to_radians();
    }

    // Handle movement
    unit.velocity = na::Vector3::new(0.0, 0.0, 0.0);

    let forward = (na::UnitQuaternion::from(unit.get_look_quat()) * na::Vector3::new(0.0, 0.0, -1.0)).component_mul(&na::Vector3::new(1.0, 0.0, 1.0)).normalize();
    let strafe = forward.cross(&na::Vector3::new(0.0, 1.0, 0.0)).normalize();

    if keypress.pressed(KeyCode::W) {
        unit.velocity += forward;
    }

    if keypress.pressed(KeyCode::S) {
        unit.velocity -= forward;
    }

    if keypress.pressed(KeyCode::A) {
        unit.velocity -= strafe;
    }

    if keypress.pressed(KeyCode::D) {
        unit.velocity += strafe;
    }

    if keypress.pressed(KeyCode::Space) && unit.is_touching_ground {
        unit.velocity.y += 1.0;
    }

    unit.velocity *= player.speed;

    if mousebutton.just_pressed(MouseButton::Left) {
        unit.shoot = true;
    }
}

fn spawn_player(mut commands: Commands) {
    commands.spawn().insert_bundle(unit::UnitBundle::new(Vec3::new(40.0, 3.0, -50.0), crate::battle::TeamId::ONE)).insert(Player::default());
}

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_startup_system(spawn_player.system().label("spawn_player"));
        app.add_system(player_controller.system());
        app.add_plugin(crate::camera::CameraPlugin);
    }
}
