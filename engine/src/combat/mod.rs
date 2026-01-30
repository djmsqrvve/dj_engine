use crate::data::components::CombatStatsComponent;
use bevy::prelude::*;

pub struct DJCombatPlugin;

impl Plugin for DJCombatPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<DamageEvent>()
            .add_systems(Update, (
                process_damage_system,
                check_death_system,
            ).chain());
    }
}

#[derive(Event, Debug)]
pub struct DamageEvent {
    pub target: Entity,
    pub amount: f32,
    pub source: Option<Entity>,
}

fn process_damage_system(
    mut events: EventReader<DamageEvent>,
    mut query: Query<&mut CombatStatsComponent>,
) {
    for event in events.read() {
        if let Ok(mut stats) = query.get_mut(event.target) {
            // Simple damage calculation
            // In a real system, we'd check defense/armor here
            stats.hp -= event.amount as i32;
            info!("Entity {:?} took {:.1} damage. HP remaining: {}", event.target, event.amount, stats.hp);
        }
    }
}

fn check_death_system(
    mut commands: Commands,
    query: Query<(Entity, &CombatStatsComponent)>,
) {
    for (entity, stats) in &query {
        if stats.hp <= 0 {
            info!("Entity {:?} has died.", entity);
            commands.entity(entity).despawn_recursive();
            // TODO: Trigger Loot drops or Death effects
        }
    }
}

/// Helper system to trigger damage from collisions (if body type allows)
pub fn trigger_combat_damage(
    mut collision_events: EventReader<crate::physics::CollisionEvent>,
    mut damage_events: EventWriter<DamageEvent>,
    stats_query: Query<&CombatStatsComponent>,
) {
     for collision in collision_events.read() {
        // If entity A has stats, check if entity B is an "attacker"
        // This is a placeholder for actual combat logic
        if let (Ok(_stats_a), Ok(stats_b)) = (stats_query.get(collision.entity_a), stats_query.get(collision.entity_b)) {
             // For prototype: objects with stats damage each other on collision
             damage_events.send(DamageEvent {
                 target: collision.entity_a,
                 amount: stats_b.damage as f32,
                 source: Some(collision.entity_b),
             });
        }
     }
}
