use crate::maps::entities::Map;
use crate::maps::terrain::TerrainType;
use crate::robot::core::types::Direction;
use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap};

pub struct Pathfinder;

impl Pathfinder {
    pub fn new() -> Self {
        Pathfinder
    }

    pub fn find_path(
        &self,
        start: (usize, usize),
        goal: (usize, usize),
        map: &Map,
    ) -> Option<Vec<(usize, usize)>> {
        #[derive(Debug)]
        struct Node {
            position: (usize, usize),
            g_cost: u32,
            f_cost: u32,
        }

        impl Ord for Node {
            fn cmp(&self, other: &Self) -> Ordering {
                other.f_cost.cmp(&self.f_cost)
            }
        }

        impl PartialOrd for Node {
            fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
                Some(self.cmp(other))
            }
        }

        impl PartialEq for Node {
            fn eq(&self, other: &Self) -> bool {
                self.f_cost == other.f_cost && self.g_cost == other.g_cost
            }
        }

        impl Eq for Node {}

        let mut open_set = BinaryHeap::new();
        let mut came_from: HashMap<(usize, usize), (usize, usize)> = HashMap::new();
        let mut g_score: HashMap<(usize, usize), u32> = HashMap::new();

        g_score.insert(start, 0);
        open_set.push(Node {
            position: start,
            g_cost: 0,
            f_cost: self.heuristic(start, goal),
        });

        while let Some(current) = open_set.pop() {
            if current.position == goal {
                let mut path = vec![current.position];
                let mut current_pos = current.position;

                while let Some(&parent) = came_from.get(&current_pos) {
                    path.push(parent);
                    current_pos = parent;
                }

                path.reverse();
                return Some(path);
            }

            for direction in Direction::all() {
                let (dx, dy) = direction.to_delta();
                let new_x = current.position.0 as i32 + dx;
                let new_y = current.position.1 as i32 + dy;

                if new_x < 0 || new_y < 0 || new_x >= map.width as i32 || new_y >= map.height as i32
                {
                    continue;
                }

                let neighbor = (new_x as usize, new_y as usize);

                let terrain_type = TerrainType::from(map.terrain[neighbor.1][neighbor.0]);
                if !terrain_type.is_traversable() {
                    continue;
                }

                let movement_cost = terrain_type.movement_cost();

                let tentative_g_score = g_score[&current.position] + movement_cost;

                if tentative_g_score < *g_score.get(&neighbor).unwrap_or(&u32::MAX) {
                    came_from.insert(neighbor, current.position);
                    g_score.insert(neighbor, tentative_g_score);

                    let f_score = tentative_g_score + self.heuristic(neighbor, goal);
                    open_set.push(Node {
                        position: neighbor,
                        g_cost: tentative_g_score,
                        f_cost: f_score,
                    });
                }
            }
        }

        None
    }

    fn heuristic(&self, a: (usize, usize), b: (usize, usize)) -> u32 {
        let dx = if a.0 > b.0 { a.0 - b.0 } else { b.0 - a.0 };
        let dy = if a.1 > b.1 { a.1 - b.1 } else { b.1 - a.1 };
        (dx + dy) as u32
    }

    pub fn get_next_move(
        &self,
        current: (usize, usize),
        target: (usize, usize),
        map: &Map,
    ) -> Option<Direction> {
        if let Some(path) = self.find_path(current, target, map) {
            if path.len() > 1 {
                let next_pos = path[1];
                let dx = next_pos.0 as i32 - current.0 as i32;
                let dy = next_pos.1 as i32 - current.1 as i32;

                match (dx, dy) {
                    (0, -1) => Some(Direction::North),
                    (0, 1) => Some(Direction::South),
                    (1, 0) => Some(Direction::East),
                    (-1, 0) => Some(Direction::West),
                    _ => None,
                }
            } else {
                None
            }
        } else {
            None
        }
    }

    pub fn manhattan_distance_to(current: (usize, usize), target: (usize, usize)) -> u32 {
        let dx = if current.0 > target.0 {
            current.0 - target.0
        } else {
            target.0 - current.0
        };
        let dy = if current.1 > target.1 {
            current.1 - target.1
        } else {
            target.1 - current.1
        };
        (dx + dy) as u32
    }

    pub fn get_safe_random_direction_from_position(
        map: &Map,
        current_x: usize,
        current_y: usize,
    ) -> Option<Direction> {
        let mut valid_directions = Vec::new();

        for direction in Direction::all() {
            let (dx, dy) = direction.to_delta();
            let new_x = current_x as i32 + dx;
            let new_y = current_y as i32 + dy;

            if new_x < 0 || new_y < 0 || new_x >= map.width as i32 || new_y >= map.height as i32 {
                continue;
            }

            let nx = new_x as usize;
            let ny = new_y as usize;

            let terrain_type = TerrainType::from(map.terrain[ny][nx]);
            if terrain_type.is_traversable() {
                valid_directions.push(direction);
            }
        }

        if valid_directions.is_empty() {
            None
        } else {
            let index = (rand::random::<f32>() * valid_directions.len() as f32) as usize;
            valid_directions.get(index).copied()
        }
    }
}