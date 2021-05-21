use rand::Rng;

use bevy::prelude::*;
use bevy_rapier3d::rapier::na;

use crate::unit;

pub struct NPC {
    pub speed: f32,
}

impl Default for NPC {
    fn default() -> Self {
        Self {
            speed: 3.0,
        }
    }
}

fn get_closest_unit(transform: &Transform, units: &Vec<(unit::UnitState, Transform)>) -> (Vec3, f32) {
    let pos = transform.translation;
    let mut min_pos = pos;
    let mut min_distance = 0.0f32;

    for (_, transform2) in units {
        let dist = pos.distance(transform2.translation);
        if min_distance == 0.0 || (dist < min_distance && dist != 0.0) {
            min_pos = transform2.translation;
            min_distance = dist;
        }
    }

    return (min_pos, min_distance);
}

fn npc_controller(
    mut units: Query<(&mut unit::UnitState, &mut Transform, Option<&NPC>)>,
) {
    let mut rng = rand::thread_rng();

    // We need a list of all units for get_closest_unit
    let mut ulist = vec![];
    for (unit, transform, _) in units.iter_mut() {
        ulist.push((unit.clone(), transform.clone()));
    }

    for (mut unit, transform, npc) in units.iter_mut() {
        // If unit is NPC, update it
        if let Some(npc) = npc {
            let (closest, dist) = get_closest_unit(&transform, &ulist);
            // If a unit is in range, point at it
            if dist > 0.0 && dist < 15.0 {
                unit.yaw = (transform.translation.x - closest.x).atan2(transform.translation.z - closest.z);

                // Chance of shooting if unit in range
                if rng.gen_range(0..30) == 0 {
                    unit.shoot = true;
                }
            } else {
                // Random direction
                if rng.gen_range(0..30) == 0 {
                    unit.yaw += ((rng.gen_range(-45..=45)) as f32).to_radians();
                }
            }

            // Handle movement
            let forward = (na::UnitQuaternion::from(unit.get_look_quat()) * na::Vector3::new(0.0, 0.0, -1.0)).component_mul(&na::Vector3::new(1.0, 0.0, 1.0)).normalize();
            // let strafe = forward.cross(&na::Vector3::new(0.0, 1.0, 0.0)).normalize();

            // Set velocity to forward dir or 0
            unit.velocity = na::Vector3::new(0.0, 0.0, 0.0);
            if rng.gen_range(0..3) != 0 {
                unit.velocity += forward * npc.speed;
            }
        }
    }
}

fn spawn_npcs(
    mut commands: Commands,

    assets: Res<AssetServer>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn()
    .insert_bundle(unit::UnitBundle::new(Vec3::new(3.0, 3.0, 0.0)))
    .insert_bundle(PbrBundle {
        mesh: assets.get_handle(format!("models/maps/monke.glb#Mesh0/Primitive0").as_str()),
        material: materials.add(Color::rgb(0.6, 0.9, 0.6).into()),
        ..Default::default()
    })
    .insert(NPC::default());
}

pub struct NpcPlugin;

impl Plugin for NpcPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_startup_system(spawn_npcs.system());
        app.add_system(npc_controller.system());
    }
}
