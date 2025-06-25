use super::robot::RobotType;
use crate::map::{Map, TerrainType};
use rand::Rng;

#[derive(Debug, Clone)]
pub enum BehaviorState {
    Exploring,
    Analyzing,
    Collecting,
    Returning,
    Waiting,
}

#[derive(Debug)]
pub struct RobotBehavior {
    state: BehaviorState,
    target: Option<(usize, usize)>,
    _robot_type: RobotType,
    step_counter: u32,
    action_counter: u32, // Compteur pour limiter les actions répétitives
}

impl RobotBehavior {
    pub fn new(robot_type: RobotType) -> Self {
        RobotBehavior {
            state: BehaviorState::Exploring,
            target: None,
            _robot_type: robot_type,
            step_counter: 0,
            action_counter: 0,
        }
    }

    pub fn update_robot_state(&mut self, x: &mut usize, y: &mut usize, energy: &mut u32, map: &mut Map, robot_type: &RobotType, robot_id: u32) {
        self.step_counter += 1;

        match self.state {
            BehaviorState::Exploring => self.explore_behavior(x, y, energy, map, robot_type, robot_id),
            BehaviorState::Analyzing => self.analyze_behavior(x, y, energy, map, robot_id),
            BehaviorState::Collecting => self.collect_behavior(x, y, energy, map, robot_id),
            BehaviorState::Returning => self.return_behavior(),
            BehaviorState::Waiting => self.wait_behavior(energy),
        }
    }

    fn explore_behavior(&mut self, x: &mut usize, y: &mut usize, energy: &mut u32, map: &mut Map, robot_type: &RobotType, robot_id: u32) {
        // Chercher des points d'intérêt à proximité
        let discoveries = self.scan_area(*x, *y, map, 3);

        if !discoveries.is_empty() && self.target.is_none() {
            // Aller vers le point d'intérêt le plus proche
            let (target_x, target_y, _) = &discoveries[0];
            self.target = Some((*target_x, *target_y));
        }

        if let Some((target_x, target_y)) = self.target {
            // Si on a atteint la cible
            if *x == target_x && *y == target_y {
                // Changer de comportement selon le type de robot
                match robot_type {
                    RobotType::Analyzer => {
                        self.state = BehaviorState::Analyzing;
                        self.action_counter = 0;
                    },
                    RobotType::Harvester => {
                        self.state = BehaviorState::Collecting;
                        self.action_counter = 0;
                    },
                    _ => {
                        // Explorer et Scout continuent d'explorer vers une nouvelle cible
                        self.target = None;
                        self.random_move(x, y, energy, map);
                    }
                }
            } else {
                // Se déplacer vers la cible
                self.move_towards_target(x, y, energy, map);
            }
        } else {
            // Mouvement aléatoire pour exploration
            self.random_move(x, y, energy, map);
        }
    }

    fn analyze_behavior(&mut self, x: &mut usize, y: &mut usize, energy: &mut u32, map: &mut Map, robot_id: u32) {
        self.action_counter += 1;

        if let Some(cell) = map.get_cell(*x, *y) {
            match cell.terrain {
                TerrainType::Interest | TerrainType::Mineral => {
                    // Analyser pendant quelques étapes seulement
                    if self.action_counter % 3 == 0 && self.action_counter < 10 {
                        println!("🔬 Robot {} analyse la zone ({},{})", robot_id, x, y);
                        *energy = energy.saturating_sub(3);
                    }
                }
                _ => {}
            }
        }

        // Retourner à l'exploration après analyse limitée
        if self.action_counter >= 8 {
            self.state = BehaviorState::Exploring;
            self.target = None;
            self.action_counter = 0;
        }
    }

    fn collect_behavior(&mut self, x: &mut usize, y: &mut usize, energy: &mut u32, map: &mut Map, robot_id: u32) {
        self.action_counter += 1;

        if let Some(cell) = map.get_cell(*x, *y) {
            match cell.terrain {
                TerrainType::Mineral => {
                    if self.action_counter % 2 == 0 && self.action_counter < 6 {
                        println!("⛏️ Robot {} collecte des minéraux à ({},{})", robot_id, x, y);
                        *energy = energy.saturating_sub(4);
                        map.deplete_resource(*x, *y); // Épuiser la ressource
                    }
                }
                TerrainType::Energy => {
                    if self.action_counter % 2 == 0 && self.action_counter < 4 {
                        println!("🔋 Robot {} collecte de l'énergie à ({},{})", robot_id, x, y);
                        *energy = (*energy + 5).min(100);
                        map.deplete_resource(*x, *y); // Épuiser la ressource
                    }
                }
                _ => {}
            }
        }

        // Retourner à l'exploration après collecte limitée
        if self.action_counter >= 5 {
            self.state = BehaviorState::Exploring;
            self.target = None;
            self.action_counter = 0;
        }
    }

    fn return_behavior(&mut self) {
        // Comportement de retour vers la base (non implémenté complètement)
        self.state = BehaviorState::Exploring;
    }

    fn wait_behavior(&mut self, energy: &mut u32) {
        // Attendre et récupérer de l'énergie
        *energy = (*energy + 1).min(100);

        if self.step_counter % 5 == 0 {
            self.state = BehaviorState::Exploring;
        }
    }

    fn move_towards_target(&self, x: &mut usize, y: &mut usize, energy: &mut u32, map: &mut Map) {
        if let Some((target_x, target_y)) = self.target {
            let dx = if target_x > *x { 1 } else if target_x < *x { -1 } else { 0 };
            let dy = if target_y > *y { 1 } else if target_y < *y { -1 } else { 0 };

            let new_x = (*x as i32 + dx).max(0) as usize;
            let new_y = (*y as i32 + dy).max(0) as usize;

            if map.is_passable(new_x, new_y) && *energy > 2 {
                *x = new_x;
                *y = new_y;
                *energy = energy.saturating_sub(1); // Coût réduit du mouvement
            }
        }
    }

    fn random_move(&self, x: &mut usize, y: &mut usize, energy: &mut u32, map: &mut Map) {
        let mut rng = rand::thread_rng();
        let directions = [(0, 1), (1, 0), (0, -1), (-1, 0), (1, 1), (-1, -1), (1, -1), (-1, 1)];
        let (dx, dy) = directions[rng.gen_range(0..directions.len())];

        let new_x = (*x as i32 + dx).max(0) as usize;
        let new_y = (*y as i32 + dy).max(0) as usize;

        if map.is_passable(new_x, new_y) && *energy > 1 {
            *x = new_x;
            *y = new_y;
            *energy = energy.saturating_sub(1); // Coût réduit du mouvement
        }
    }

    fn scan_area(&self, x: usize, y: usize, map: &Map, range: usize) -> Vec<(usize, usize, String)> {
        let mut discoveries = Vec::new();

        for dy in -(range as i32)..=(range as i32) {
            for dx in -(range as i32)..=(range as i32) {
                let scan_x = (x as i32 + dx).max(0) as usize;
                let scan_y = (y as i32 + dy).max(0) as usize;

                if let Some(cell) = map.get_cell(scan_x, scan_y) {
                    if cell.terrain.exploration_value() > 0 && !cell.depleted {
                        discoveries.push((scan_x, scan_y, format!("{:?}", cell.terrain)));
                    }
                }
            }
        }

        discoveries
    }

    pub fn get_current_state(&self) -> &BehaviorState {
        &self.state
    }
}