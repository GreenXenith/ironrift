use bevy::prelude::*;
use bevy_rapier3d::rapier::na;

pub struct Bullet {
    pub age: f32,
    pub lifetime: f32,
}

impl Default for Bullet {
    fn default() -> Self {
        Self {
            age: 0.0,
            lifetime: 2.0,
        }
    }
}

fn bullet_handler(
    mut commands: Commands,
    time: Res<Time>,
    mut bquery: Query<(&mut Bullet, Entity)>,
) {
    for (mut bullet, entity) in bquery.iter_mut() {
        bullet.age += time.delta_seconds();
        if bullet.age > bullet.lifetime {
            commands.entity(entity).despawn();
        }
    }
}

#[derive(Bundle)]
pub struct BulletBundle {
    pub bullet: Bullet,
    pub rigidbodybuilder: bevy_rapier3d::rapier::dynamics::RigidBodyBuilder,
}

impl BulletBundle {
    pub fn new(position: Vec3, rotation: Quat, velocity: Vec3) -> BulletBundle {
        BulletBundle {
            bullet: Bullet::default(),
            rigidbodybuilder: bevy_rapier3d::rapier::dynamics::RigidBodyBuilder::new_dynamic()
                .position(na::Isometry::from_parts(na::Translation3::from(na::Vector3::from(position)), rotation.into()))
                .linvel(velocity.x, velocity.y, velocity.z)
                .gravity_scale(0.0),
        }
    }
}

impl Default for BulletBundle {
    fn default() -> Self {
        Self::new(Vec3::ZERO, Quat::IDENTITY, Vec3::ZERO)
    }
}

pub struct BulletPlugin;

impl Plugin for BulletPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_system(bullet_handler.system());
    }
}
