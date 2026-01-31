impl Plugin for DJCombatPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<ActiveCombat>()
            .add_message::<DamageEvent>()
            .add_message::<CombatActionEvent>()
            .add_systems(Update, (
                process_damage_system,
                check_death_system,
                update_combat_queue,
                process_battle_triggers,
                process_combat_turns,
            ).chain());
    }
}

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq, Reflect)]
pub enum Team {
    Player,
    Enemy,
}

#[derive(Resource, Default, Debug, Reflect)]
#[reflect(Resource)]
pub struct ActiveCombat {
    pub is_active: bool,
    pub turn_order: Vec<Entity>,
    pub current_turn_index: usize,
    pub round: u32,
}

#[derive(Message, Debug, Clone)]
pub struct CombatActionEvent {
    pub source: Entity,
    pub target: Entity,
    pub action_id: String,
}

#[derive(Message, Debug, Clone)]
pub struct DamageEvent {
    pub target: Entity,
    pub amount: f32,
    pub source: Option<Entity>,
}

fn process_damage_system(
    mut events: MessageReader<DamageEvent>,
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
    mut combat: ResMut<ActiveCombat>,
) {
    for (entity, stats) in &query {
        if stats.hp <= 0 {
            info!("Entity {:?} has died.", entity);
            commands.entity(entity).despawn();
            
            // Remove from combat if present
            if let Some(pos) = combat.turn_order.iter().position(|&e| e == entity) {
                combat.turn_order.remove(pos);
                if pos <= combat.current_turn_index && combat.current_turn_index > 0 {
                    combat.current_turn_index -= 1;
                }
            }
        }
    }
}

fn update_combat_queue(
    mut combat: ResMut<ActiveCombat>,
) {
    if !combat.is_active || combat.turn_order.is_empty() { return; }

    if combat.current_turn_index >= combat.turn_order.len() {
        combat.current_turn_index = 0;
        combat.round += 1;
        info!("--- Combat Round {} Start ---", combat.round);
    }
}

fn process_battle_triggers(
    mut flow_events: MessageReader<crate::story_graph::events::StoryFlowEvent>,
    mut combat: ResMut<ActiveCombat>,
    combatants: Query<(Entity, &CombatStatsComponent)>,
) {
    for event in flow_events.read() {
        if let crate::story_graph::events::StoryFlowEvent::StartBattle { enemy_id } = event {
            info!("Initializing combat encounter with: {}", enemy_id);
            
            // 1. Reset combat state
            combat.is_active = true;
            combat.round = 1;
            combat.current_turn_index = 0;
            combat.turn_order.clear();

            // 2. Identify participants
            // For now, let's just grab everyone with CombatStatsComponent
            // In a real game, we'd filter by vicinity or "Team"
            let mut participants: Vec<(Entity, i32)> = combatants.iter()
                .map(|(e, stats)| (e, stats.speed))
                .collect();

            // 3. Roll Initiative (Sorted by Speed DESC)
            participants.sort_by(|a, b| b.1.cmp(&a.1));
            combat.turn_order = participants.into_iter().map(|(e, _)| e).collect();

            info!("Combat Order: {:?}", combat.turn_order);
        }
    }
}

fn process_combat_turns(
    mut combat: ResMut<ActiveCombat>,
    teams: Query<&Team>,
    mut damage_events: MessageWriter<DamageEvent>,
    lua_ctx: Res<crate::lua_scripting::LuaContext>,
) {
    if !combat.is_active || combat.turn_order.is_empty() { return; }

    let current_entity = combat.turn_order[combat.current_turn_index];
    
    // Check if entity still exists
    if let Ok(team) = teams.get(current_entity) {
        match team {
            Team::Player => {
                // For the "Hard" prototype, let's make the player "Attack" the first enemy
                // In a real CLI game, we'd wait for a command like "attack"
                info!("Player's Turn - Waiting for action...");
            }
            Team::Enemy => {
                // Trigger Lua AI if it exists
                if let Ok(lua) = lua_ctx.lua.lock() {
                    let globals = lua.globals();
                    if let Ok(func) = globals.get::<_, mlua::Function>("on_enemy_turn") {
                        if let Err(e) = func.call::<_, ()>(()) {
                            error!("Error in Lua enemy AI: {}", e);
                        }
                    } else {
                        // Default AI: attack first player
                        info!("Enemy's Turn - Auto-attacking...");
                        // Find first player
                        // This is a placeholder for actual target selection
                    }
                }
            }
        }
    } else {
        // Entity might have been despawned
        combat.current_turn_index += 1;
    }
}

/// Helper system to trigger damage from collisions (if body type allows)
pub fn trigger_combat_damage(
    mut collision_events: MessageReader<crate::physics::CollisionEvent>,
    mut damage_events: MessageWriter<DamageEvent>,
    stats_query: Query<&CombatStatsComponent>,
) {
     for collision in collision_events.read() {
        // If entity A has stats, check if entity B is an "attacker"
        // This is a placeholder for actual combat logic
        if let (Ok(_stats_a), Ok(stats_b)) = (stats_query.get(collision.a), stats_query.get(collision.b)) {
             // For prototype: objects with stats damage each other on collision
             damage_events.write(DamageEvent {
                 target: collision.a,
                 amount: stats_b.damage as f32,
                 source: Some(collision.b),
             });
        }
     }
}
