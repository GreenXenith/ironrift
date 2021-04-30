use bevy::prelude::*;
use bevy::input::{keyboard::KeyCode};
use bevy_rapier3d::{physics::{EventQueue, RigidBodyHandleComponent}, rapier::{dynamics::{RigidBodyBuilder, RigidBodySet}, geometry::{ColliderSet, ContactEvent}}};
use bevy_rapier3d::rapier::geometry::ColliderBuilder;

pub struct Player {
    pub is_touching_ground: bool,
}

impl Default for Player {
    fn default() -> Self {
        Self {
            is_touching_ground: false,
        }
    }
}

fn player_controller(
    keys: Res<Input<KeyCode>>,
    events: Res<EventQueue>,
    mut bodies: ResMut<RigidBodySet>,
    colliders: Res<ColliderSet>,

    mut query: Query<(&mut Player, &RigidBodyHandleComponent)>,
) {
    let (mut player, handle) = query.single_mut().unwrap();

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

    let body = bodies.get_mut(handle.handle()).unwrap();
    let mut velocity = Vec3::ZERO;
    let speed = 5.0;

    if keys.pressed(KeyCode::Up) {
        velocity.z -= speed;
    }

    if keys.pressed(KeyCode::Down) {
        velocity.z += speed;
    }

    if keys.pressed(KeyCode::Left) {
        velocity.x -= speed;
    }

    if keys.pressed(KeyCode::Right) {
        velocity.x += speed;
    }

    if keys.pressed(KeyCode::RShift) && player.is_touching_ground {
        velocity.y += 5.0;
    }

    velocity.y += body.linvel().y;

    body.set_linvel(bevy_rapier3d::na::Vector3::new(velocity.x, velocity.y, velocity.z), true);
}

fn spawn_player(
    mut commands: Commands,
    mut materials: ResMut<Assets<StandardMaterial>>,
    assets: Res<AssetServer>,
) {
    commands.spawn().insert_bundle(PbrBundle {
        mesh: assets.get_handle("models/maps/monke.glb#Mesh0/Primitive0"),
        material: materials.add(Color::WHITE.into()),
        ..Default::default()
    })
    .insert(RigidBodyBuilder::new_dynamic().translation(0.0, 3.0, 0.0).lock_rotations())
    .insert(ColliderBuilder::capsule_y(0.5, 1.0).user_data(2))
    .insert(Player::default());
}

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
	fn build(&self, app: &mut AppBuilder) {
        app.add_startup_system(spawn_player.system());
		app.add_system(player_controller.system());
	}
}
