use bevy::prelude::*;
use bevy::input::keyboard::KeyCode;
use bevy_rapier3d::rapier;
use bevy_rapier3d::rapier::na;
use bevy_rapier3d::rapier::geometry;

pub struct Player {
    pub is_touching_ground: bool,
    pub yaw: f32,
    pub pitch: f32,
    pub velocity: na::Vector3<f32>,
    pub speed: f32,
    pub sensitivity: f32,
}

impl Default for Player {
    fn default() -> Self {
        Self {
            is_touching_ground: false,
            yaw: 0.0,
            pitch: 0.0,
            velocity: na::Vector3::new(0.0, 0.0, 0.0),
            speed: 5.0,
            sensitivity: 10.0,
        }
    }
}

fn player_controller(
    mut commands: Commands,

    time: Res<Time>,
    mut mousemotion: EventReader<bevy::input::mouse::MouseMotion>,
    mousebutton: Res<Input<bevy::input::mouse::MouseButton>>,
    keypress: Res<Input<KeyCode>>,

    assets: Res<AssetServer>,
    mut materials: ResMut<Assets<StandardMaterial>>,

    events: Res<bevy_rapier3d::physics::EventQueue>,
    mut bodies: ResMut<rapier::dynamics::RigidBodySet>,
    colliders: Res<geometry::ColliderSet>,

    mut pquery: Query<(&mut Player, &bevy_rapier3d::physics::RigidBodyHandleComponent)>,
) {
    let (mut player, handle) = pquery.single_mut().unwrap();

    // Check for contact
    if events.contact_events.len() > 0 {
        match events.contact_events.pop().unwrap() {
            geometry::ContactEvent::Started(handle1, handle2) => {
                let type1 = colliders.get(handle1).unwrap().user_data;
                let type2 = colliders.get(handle2).unwrap().user_data;
                if type1 * type2 == 2 {
                    player.is_touching_ground = true;
                }
            }

            geometry::ContactEvent::Stopped(handle1, handle2) => {
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

    for event in mousemotion.iter() {
        delta_m += event.delta;
    }

    if !delta_m.is_nan() {
        player.yaw -= delta_m.x * player.sensitivity * delta_s;
        player.pitch += delta_m.y * player.sensitivity * delta_s;
        player.pitch = player.pitch.clamp(-89.9, 89.9);

        let mut new_pos = body.position().clone();
        new_pos.rotation = na::UnitQuaternion::new(na::Vector3::new(0.0, player.yaw.to_radians(), 0.0));
        body.set_position(new_pos, true);
    }

    // Handle movement
    player.velocity = na::Vector3::new(0.0, 0.0, 0.0);

    let forward = (body.position().rotation * na::Vector3::new(0.0, 0.0, 1.0)).component_mul(&na::Vector3::new(1.0, 0.0, 1.0)).normalize();
    let strafe = forward.cross(&na::Vector3::new(0.0, 1.0, 0.0)).normalize();
    
    if keypress.pressed(KeyCode::W) {
        player.velocity -= forward;
    }

    if keypress.pressed(KeyCode::S) {
        player.velocity += forward;
    }

    if keypress.pressed(KeyCode::A) {
        player.velocity += strafe;
    }

    if keypress.pressed(KeyCode::D) {
        player.velocity -= strafe;
    }

    if keypress.pressed(KeyCode::Space) && player.is_touching_ground {
        player.velocity.y += 1.0;
    }

    let speed = player.speed;
    player.velocity *= speed;

    player.velocity.y += body.linvel().y;

    body.set_linvel(player.velocity, true);

    if mousebutton.just_pressed(MouseButton::Left) {
        let (pitch, yaw) = (-player.pitch.to_radians(), player.yaw.to_radians());
        let rot = Quat::from_rotation_ypr(yaw, pitch, 0.0);
        let dir = rot.mul_vec3(Vec3::new(0.0, 0.0, -1.0)).normalize();
        let tra = body.position().translation;
        let pos = Vec3::new(tra.x, tra.y, tra.z) + dir * Vec3::new(2.0, 2.0, 2.0);
        let speed = 300.0;

        commands.spawn().insert_bundle(PbrBundle {
            mesh: assets.get_handle(format!("models/maps/monke.glb#Mesh0/Primitive0").as_str()),
            material: materials.add(Color::rgb(0.6, 0.9, 0.6).into()),
            ..Default::default()
        })
        .insert_bundle(crate::bullet::BulletBundle::new(pos, rot, dir * Vec3::new(speed, speed, speed)));
    }
}

fn spawn_player(
    mut commands: Commands,
) {
    commands.spawn().insert_bundle((
        Transform::default(),
        rapier::dynamics::RigidBodyBuilder::new_dynamic().translation(0.0, 3.0, 0.0).lock_rotations(),
        geometry::ColliderBuilder::capsule_y(0.5, 1.0).user_data(2),
        Player::default(),
    ));
}

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_startup_system(spawn_player.system());
        app.add_system(player_controller.system());
        app.add_plugin(crate::camera::CameraPlugin);
        app.add_plugin(crate::bullet::BulletPlugin);
    }
}
