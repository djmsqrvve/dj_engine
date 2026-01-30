use crate::data::scene::PathfindingGrid;
use bevy::prelude::*;
use std::collections::{BinaryHeap, HashMap};

pub struct DJNavigationPlugin;

impl Plugin for DJNavigationPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, update_navigation_agents);
    }
}

#[derive(Component, Debug, Clone, Reflect, Default)]
#[reflect(Component)]
pub struct NavAgent {
    pub target: Option<Vec2>,
    pub current_path: Vec<Vec2>,
    pub speed: f32,
}

fn update_navigation_agents(
    mut query: Query<(&mut Transform, &mut NavAgent)>,
    grid: Option<Res<PathfindingGrid>>,
    time: Res<Time>,
) {
    let Some(grid) = grid else { return };
    
    for (mut transform, mut agent) in &mut query {
        if let Some(target) = agent.target {
            // If we have no path, calculate it
            if agent.current_path.is_empty() {
                let start = transform.translation.truncate();
                agent.current_path = find_path(&grid, start, target);
            }

            // Follow path
            if let Some(next_point) = agent.current_path.get(0).copied() {
                let current_pos = transform.translation.truncate();
                let dir = (next_point - current_pos).normalize();
                let distance = (next_point - current_pos).length();
                
                let move_amount = agent.speed * time.delta_secs();
                if move_amount >= distance {
                    transform.translation = next_point.extend(transform.translation.z);
                    agent.current_path.remove(0);
                } else {
                    transform.translation += (dir * move_amount).extend(0.0);
                }
            } else {
                // Reached target
                agent.target = None;
            }
        }
    }
}

// Simple A* Search
fn find_path(grid: &PathfindingGrid, start_world: Vec2, end_world: Vec2) -> Vec<Vec2> {
    let start = world_to_grid(start_world, grid);
    let end = world_to_grid(end_world, grid);

    if !is_traversable(grid, end) {
        return vec![];
    }

    let mut open_set = BinaryHeap::new();
    open_set.push(Node { pos: start, cost: 0 });

    let mut came_from: HashMap<(i32, i32), (i32, i32)> = HashMap::new();
    let mut g_score: HashMap<(i32, i32), i32> = HashMap::new();
    g_score.insert(start, 0);

    while let Some(current_node) = open_set.pop() {
        let current = current_node.pos;
        if current == end {
            return reconstruct_path(came_from, end, grid);
        }

        for neighbor in get_neighbors(current) {
            if !is_traversable(grid, neighbor) { continue; }

            let tentative_g_score = g_score[&current] + 1;
            if tentative_g_score < *g_score.get(&neighbor).unwrap_or(&i32::MAX) {
                came_from.insert(neighbor, current);
                g_score.insert(neighbor, tentative_g_score);
                let f_score = tentative_g_score + heuristic(neighbor, end);
                open_set.push(Node { pos: neighbor, cost: f_score });
            }
        }
    }

    vec![]
}

fn world_to_grid(pos: Vec2, grid: &PathfindingGrid) -> (i32, i32) {
    (pos.x as i32 / grid.cell_size as i32, pos.y as i32 / grid.cell_size as i32)
}

fn grid_to_world(pos: (i32, i32), grid: &PathfindingGrid) -> Vec2 {
    let half_cell = grid.cell_size as f32 * 0.5;
    Vec2::new(pos.0 as f32 * grid.cell_size as f32 + half_cell, pos.1 as f32 * grid.cell_size as f32 + half_cell)
}

fn is_traversable(grid: &PathfindingGrid, pos: (i32, i32)) -> bool {
    if pos.0 < 0 || pos.0 >= grid.width as i32 || pos.1 < 0 || pos.1 >= grid.height as i32 {
        return false;
    }
    let index = (pos.1 as u32 * grid.width + pos.0 as u32) as usize;
    grid.cells.get(index).map_or(false, |c| c.walkable) // Using walkable instead of !blocked as defined in PathfindingCell
}

fn get_neighbors(pos: (i32, i32)) -> Vec<(i32, i32)> {
    vec![
        (pos.0 + 1, pos.1), (pos.0 - 1, pos.1),
        (pos.0, pos.1 + 1), (pos.0, pos.1 - 1),
    ]
}

fn heuristic(a: (i32, i32), b: (i32, i32)) -> i32 {
    (a.0 - b.0).abs() + (a.1 - b.1).abs()
}

fn reconstruct_path(came_from: HashMap<(i32, i32), (i32, i32)>, mut current: (i32, i32), grid: &PathfindingGrid) -> Vec<Vec2> {
    let mut path = vec![grid_to_world(current, grid)];
    while let Some(&prev) = came_from.get(&current) {
        current = prev;
        path.push(grid_to_world(current, grid));
    }
    path.reverse();
    path
}

#[derive(Copy, Clone, Eq, PartialEq)]
struct Node {
    pos: (i32, i32),
    cost: i32,
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.cost.cmp(&self.cost) // Min-heap
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
