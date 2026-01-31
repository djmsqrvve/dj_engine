use crate::data::components::{CollisionComponent, CollisionShape, BodyType};
use bevy::prelude::*;


/// Default collision box size in pixels (32x32).
/// TODO(#108): Get collision size from actual sprite/component data instead of default.
const DEFAULT_COLLISION_SIZE: f32 = 32.0;

/// Separation distance to apply when resolving collisions.
const COLLISION_SEPARATION: f32 = 2.0;

pub struct DJPhysicsPlugin;

impl Plugin for DJPhysicsPlugin {
    fn build(&self, app: &mut App) {
        app.add_message::<CollisionEvent>()
            .add_systems(Update, (
                update_collision_system,
                resolve_collision_system.after(update_collision_system),
            ));
    }
}

#[derive(Message, Debug, Clone)]
pub struct CollisionEvent {
    pub a: Entity,
    pub b: Entity,
}

fn update_collision_system(
    mut collision_events: MessageWriter<CollisionEvent>,
    query: Query<(Entity, &Transform, &CollisionComponent)>,
) {
    let entities: Vec<_> = query.iter().collect();
    
    for i in 0..entities.len() {
        for j in i + 1..entities.len() {
            let (entity_a, transform_a, collision_a) = entities[i];
            let (entity_b, transform_b, collision_b) = entities[j];

            if check_collision(transform_a, collision_a, transform_b, collision_b) {
                collision_events.write(CollisionEvent {
                    a: entity_a,
                    b: entity_b,
                });
            }
        }
    }
}

fn check_collision(
    t1: &Transform, c1: &CollisionComponent,
    t2: &Transform, c2: &CollisionComponent,
) -> bool {
    match (&c1.shape, &c2.shape) {
        (CollisionShape::Box, CollisionShape::Box) => {
            let pos1 = t1.translation.truncate();
            let pos2 = t2.translation.truncate();
            let size1 = Vec2::ONE * DEFAULT_COLLISION_SIZE;
            let size2 = Vec2::ONE * DEFAULT_COLLISION_SIZE;

            let min1 = pos1 - size1 * 0.5;
            let max1 = pos1 + size1 * 0.5;
            let min2 = pos2 - size2 * 0.5;
            let max2 = pos2 + size2 * 0.5;

            max1.x > min2.x && min1.x < max2.x &&
            max1.y > min2.y && min1.y < max2.y
        }
        // Circle collision not yet implemented.
        _ => false,
    }
}

fn resolve_collision_system(
    mut query: Query<(&mut Transform, &CollisionComponent)>,
    mut events: MessageReader<CollisionEvent>,
) {
    for event in events.read() {
        // Simple static response: prevent overlap if one is static
        if let Ok([q1, q2]) = query.get_many_mut([event.a, event.b]) {
            let (mut t1, c1) = q1;
            let (mut t2, c2) = q2;

            if c1.body_type == BodyType::Static && c2.body_type == BodyType::Dynamic {
                let diff = t2.translation - t1.translation;
                if diff != Vec3::ZERO {
                    t2.translation += diff.normalize() * COLLISION_SEPARATION;
                }
            } else if c1.body_type == BodyType::Dynamic && c2.body_type == BodyType::Static {
                let diff = t1.translation - t2.translation;
                if diff != Vec3::ZERO {
                    t1.translation += diff.normalize() * COLLISION_SEPARATION;
                }
            }
        }
    }
}
