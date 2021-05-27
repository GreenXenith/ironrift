use std::borrow::BorrowMut;

use bevy::prelude::*;
use bevy::app::Events;
use bevy_rapier3d::physics;
use bevy_rapier3d::rapier;
use bevy_rapier3d::rapier::na;
use bevy_rapier3d::rapier::geometry;

// Unit-specific data
#[derive(Clone, Copy)]
pub struct UnitState {
    pub is_touching_ground: bool,
    pub yaw: f32,
    pub pitch: f32,
    pub roll: f32,
    pub velocity: na::Vector3<f32>,
    pub shoot: bool,
    pub team: crate::battle::TeamId,
    pub hp: i32,
}

impl UnitState {
    pub fn get_look_quat(&self) -> Quat {
        Quat::from_rotation_ypr(self.yaw, self.pitch, self.roll)
    }
}

impl Default for UnitState {
    fn default() -> Self {
        Self {
            is_touching_ground: false,
            yaw: 0.0,
            pitch: 0.0,
            roll: 0.0,
            velocity: na::Vector3::new(0.0, 0.0, 0.0),
            shoot: false,
            team: crate::battle::TeamId::NONE,
            hp: 3,
        }
    }
}

// Bundle for units including physics object and position
#[derive(Bundle)]
pub struct UnitBundle {
    pub state: UnitState,
    pub transform: Transform,
    pub rigidbody: rapier::dynamics::RigidBodyBuilder,
    pub collider: geometry::ColliderBuilder,
}

impl UnitBundle {
    pub fn new(position: Vec3, team: crate::battle::TeamId) -> UnitBundle {
        UnitBundle {
            state: UnitState {team: team, ..Default::default()},
            transform: Transform::default(),
            rigidbody: rapier::dynamics::RigidBodyBuilder::new_dynamic().translation(position.x, position.y, position.z).lock_rotations(),
            collider: geometry::ColliderBuilder::capsule_y(0.5, 1.0).user_data(crate::ObjectType::Unit as u128),
        }
    }
}

impl Default for UnitBundle {
    fn default() -> Self {
        Self::new(Vec3::ZERO, crate::battle::TeamId::NONE)
    }
}

// Unit updates every frame based on state
fn unit_handler(
    mut commands: Commands,

    events: Res<bevy_rapier3d::physics::EventQueue>,
    mut bodies: ResMut<rapier::dynamics::RigidBodySet>,
    colliders: Res<geometry::ColliderSet>,

    assets: Res<AssetServer>,
    mut materials: ResMut<Assets<ColorMaterial>>,

    mut exit: ResMut<Events<bevy::app::AppExit>>,

    mut query: Query<(Entity, &mut UnitState, &physics::RigidBodyHandleComponent, &physics::ColliderHandleComponent, Option<&crate::player::Player>)>,
) {
    let mut units = std::collections::HashMap::new();

    // Loop through all units and apply updates
    for (entity, mut unit, body_handle, collider_handle, player) in query.iter_mut() {
        let body = bodies.get_mut(body_handle.handle()).unwrap();

        // Update rotation
        body.set_position(bevy_rapier3d::rapier::math::Isometry::from_parts(body.position().translation, unit.get_look_quat().into()), true);

        // Apply velocity
        unit.velocity.y += body.linvel().y;
        body.set_linvel(unit.velocity, true);

        // Fire
        if unit.shoot {
            let dir = unit.get_look_quat().mul_vec3(Vec3::new(0.0, 0.0, -1.0)).normalize();
            let tra = body.position().translation;
            let pos = Vec3::new(tra.x, tra.y, tra.z) + dir * Vec3::new(2.0, 2.0, 2.0);
            let speed = 300.0;

            commands.spawn().insert_bundle(SpriteBundle {
                sprite: bevy::sprite::Sprite {
                    size: Vec2::new(5.0, 5.0),
                    resize_mode: bevy::sprite::SpriteResizeMode::Manual,
                    ..Default::default()
                },
                material: materials.add(assets.load("bullet.png").into()),
                ..Default::default()
            })
            .insert_bundle(crate::bullet::BulletBundle::new(pos, unit.get_look_quat(), dir * Vec3::new(speed, speed, speed)));

            unit.shoot = false;
        }

        // Add available units to unit list
        units.insert(collider_handle.handle(), (unit, entity, player));
    }

    // Check for unit contacts
    let terrain = crate::ObjectType::Terrain as u128;
    while let Ok(contact_event) = events.contact_events.pop() {
        match contact_event {
            geometry::ContactEvent::Started(handle1, handle2) => {
                if units.contains_key(&handle1) || units.contains_key(&handle2) {
                    let unit_handle = if units.contains_key(&handle1) { handle1 } else { handle2 };
                    let other_handle = if unit_handle == handle1 { handle2 } else { handle1 };
                    if colliders.contains(other_handle) {
                        let otype = colliders.get(if unit_handle == handle1 { handle2 } else { handle1 }).unwrap().user_data;
                        let (unit, entity, player) = units.get_mut(&unit_handle).unwrap().borrow_mut();
                        if otype == terrain {
                            unit.is_touching_ground = true;
                        } else if otype == crate::ObjectType::Bullet as u128 {
                            unit.hp -= 1;
                            if unit.hp <= 0 {
                                if player.is_some() {
                                    println!("You died.");
                                    exit.send(bevy::app::AppExit);
                                } else {
                                    commands.entity(*entity).despawn();
                                }
                            }
                        }
                    }
                }
            }

            geometry::ContactEvent::Stopped(handle1, handle2) => {
                if units.contains_key(&handle1) || units.contains_key(&handle2) {
                    let unit_handle = if units.contains_key(&handle1) { handle1 } else { handle2 };
                    let other_handle = if unit_handle == handle1 { handle2 } else { handle1 };
                    if colliders.contains(other_handle) && colliders.get(other_handle).unwrap().user_data == terrain {
                        let (unit, ..) = units.get_mut(&unit_handle).unwrap().borrow_mut();
                        unit.is_touching_ground = false;
                    }
                }
            }
        }
    }
}

pub struct UnitPlugin;

impl Plugin for UnitPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_system(unit_handler.system());
        app.add_plugin(crate::bullet::BulletPlugin);
    }
}
