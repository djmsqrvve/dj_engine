use crate::data::components::{CollisionComponent, CollisionShape, BodyType};
use bevy::prelude::*;

pub struct DJPhysicsPlugin;

impl Plugin for DJPhysicsPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<CollisionEvent>()
            .add_systems(Update, (
                update_collision_system,
                resolve_collision_system,
            ).chain());
    }
}

#[derive(Event, Debug)]
pub struct CollisionEvent {
    pub entity_a: Entity,
    pub entity_b: Entity,
}

fn update_collision_system(
    mut collision_events: EventWriter<CollisionEvent>,
    query: Query<(Entity, &Transform, &CollisionComponent)>,
) {
    let entities: Vec<_> = query.iter().collect();
    
    for i in 0..entities.len() {
        for j in i + 1..entities.len() {
            let (entity_a, transform_a, collision_a) = entities[i];
            let (entity_b, transform_b, collision_b) = entities[j];

            if check_collision(transform_a, collision_a, transform_b, collision_b) {
                collision_events.send(CollisionEvent {
                    entity_a,
                    entity_b,
                });
            }
        }
    }
}

fn check_collision(
    t1: &Transform, c1: &CollisionComponent,
    t2: &Transform, c2: &CollisionComponent,
) -> bool {
    // Basic AABB check for now
    match (&c1.shape, &c2.shape) {
        (CollisionShape::Box, CollisionShape::Box) => {
            let pos1 = t1.translation.truncate();
            let pos2 = t2.translation.truncate();
            let size1 = Vec2::ONE * 32.0; // TODO: Get size from sprite/component
            let size2 = Vec2::ONE * 32.0;

            let min1 = pos1 - size1 * 0.5;
            let max1 = pos1 + size1 * 0.5;
            let min2 = pos2 - size2 * 0.5;
            let max2 = pos2 + size2 * 0.5;

            max1.x > min2.x && min1.x < max2.x &&
            max1.y > min2.y && min1.y < max2.y
        }
        _ => false, // TODO: Support Circle
    }
}

fn resolve_collision_system(
    mut query: Query<(&mut Transform, &CollisionComponent)>,
    mut events: EventReader<CollisionEvent>,
) {
    for event in events.read() {
        // Simple static response: prevent overlap if one is static
        if let Ok([q1, q2]) = query.get_many_mut([event.entity_a, event.entity_b]) {
            let (mut t1, c1) = q1;
            let (mut t2, c2) = q2;

            if c1.body_type == BodyType::Static && c2.body_type == BodyType::Dynamic {
                // Push t2 away from t1
                let diff = t2.translation - t1.translation;
                if diff != Vec3::ZERO {
                    t2.translation += diff.normalize() * 2.0;
                }
            } else if c1.body_type == BodyType::Dynamic && c2.body_type == BodyType::Static {
                // Push t1 away from t2
                let diff = t1.translation - t2.translation;
                if diff != Vec3::ZERO {
                    t1.translation += diff.normalize() * 2.0;
                }
            }
        }
    }
}
