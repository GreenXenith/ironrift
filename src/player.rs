use bevy::prelude::*;
use bevy::input::{keyboard::KeyCode, mouse::MouseMotion};
use bevy_rapier3d::{physics::{EventQueue, RigidBodyHandleComponent}, rapier::{dynamics::{RigidBodyBuilder, RigidBodySet}, geometry::{ColliderSet, ContactEvent}}};
use bevy_rapier3d::rapier::{na, geometry::ColliderBuilder};

pub struct Player {
    pub is_touching_ground: bool,
    pub yaw: f32,
    pub velocity: na::Vector3<f32>,
    pub speed: f32,
    pub sensitivity: f32,
}

impl Default for Player {
    fn default() -> Self {
        Self {
            is_touching_ground: false,
            yaw: 0.0,
            velocity: na::Vector3::new(0.0, 0.0, 0.0),
            speed: 5.0,
            sensitivity: 10.0,
        }
    }
}

fn player_controller(
    time: Res<Time>,
    keys: Res<Input<KeyCode>>,
    events: Res<EventQueue>,
    mut bodies: ResMut<RigidBodySet>,
    colliders: Res<ColliderSet>,

    mut mouse: EventReader<MouseMotion>,
    mut pquery: Query<(&mut Player, &RigidBodyHandleComponent)>,
) {
    let (mut player, handle) = pquery.single_mut().unwrap();

    // Check for contact
    if events.contact_events.len() > 0 {
        match events.contact_events.pop().unwrap() {
            ContactEvent::Started(handle1, handle2) => {
                let type1 = colliders.get(handle1).unwrap().user_data;
                let type2 = colliders.get(handle2).unwrap().user_data;
                if type1 * type2 == 2 {
                    player.is_touching_ground = true;
                }
            }

            ContactEvent::Stopped(handle1, handle2) => {
                let type1 = colliders.get(handle1).unwrap().user_data;
                let type2 = colliders.get(handle2).unwrap().user_data;
                if type1 * type2 == 2 {
                    player.is_touching_ground = false;
                }
            }
        }
    }

    // Get physics object
    let body = bodies.get_mut(handle.handle()).unwrap();

    let delta_s = time.delta_seconds();
    let mut delta_m: Vec2 = Vec2::ZERO;

    for event in mouse.iter() {
        delta_m += event.delta;
    }

    if !delta_m.is_nan() {
        player.yaw -= delta_m.x * player.sensitivity * delta_s;

        let mut new_pos = body.position().clone();
        new_pos.rotation = na::UnitQuaternion::new(na::Vector3::new(0.0, player.yaw.to_radians(), 0.0));
        body.set_position(new_pos, true);
    }

    // Handle movement
    player.velocity = na::Vector3::new(0.0, 0.0, 0.0);

    let forward = (body.position().rotation * na::Vector3::new(0.0, 0.0, 1.0)).component_mul(&na::Vector3::new(1.0, 0.0, 1.0)).normalize();
    let strafe = forward.cross(&na::Vector3::new(0.0, 1.0, 0.0)).normalize();
    
    if keys.pressed(KeyCode::W) {
        player.velocity -= forward;
    }

    if keys.pressed(KeyCode::S) {
        player.velocity += forward;
    }

    if keys.pressed(KeyCode::A) {
        player.velocity += strafe;
    }

    if keys.pressed(KeyCode::D) {
        player.velocity -= strafe;
    }

    if keys.pressed(KeyCode::Space) && player.is_touching_ground {
        player.velocity.y += 1.0;
    }

    let speed = player.speed;
    player.velocity *= speed;

    player.velocity.y += body.linvel().y;

    body.set_linvel(player.velocity, true);
}

fn spawn_player(
    mut commands: Commands,
) {
    commands.spawn().insert_bundle((
        Transform::default(),
        RigidBodyBuilder::new_dynamic().translation(0.0, 3.0, 0.0).lock_rotations(),
        ColliderBuilder::capsule_y(0.5, 1.0).user_data(2),
        Player::default(),
    ));
}

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_startup_system(spawn_player.system());
        app.add_system(player_controller.system());
    }
}
