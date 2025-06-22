// src/robot/behavior.rs - Comportements des robots améliorés
use crate::Position;

#[derive(Debug, Clone)]
pub enum RobotAction {
    Move(Position),
    Collect,
    Analyze,
    Communicate(Vec<usize>), // IDs des robots à contacter
    Wait,
}

pub struct BehaviorEngine;

impl BehaviorEngine {
    pub fn decide_action(
        robot_behavior: crate::RobotBehavior,
        current_position: Position,
        energy: u32,
        inventory_full: bool,
    ) -> RobotAction {
        match robot_behavior {
            crate::RobotBehavior::Explorateur => {
                if energy < 10 {
                    RobotAction::Wait // Récupérer de l'énergie
                } else {
                    // Mouvement intelligent : exploration en spirale
                    let directions = [
                        (1, 0), (0, 1), (-1, 0), (0, -1), // Cardinal
                        (1, 1), (-1, 1), (-1, -1), (1, -1) // Diagonal
                    ];

                    let choice = (current_position.x + current_position.y + energy as usize) % directions.len();
                    let (dx, dy) = directions[choice];

                    let new_x = (current_position.x as i32 + dx).max(0) as usize;
                    let new_y = (current_position.y as i32 + dy).max(0) as usize;

                    RobotAction::Move(Position::new(new_x, new_y))
                }
            },
            crate::RobotBehavior::Collecteur => {
                if inventory_full {
                    // Retourner vers le point de départ pour décharger
                    if current_position.x > 1 || current_position.y > 1 {
                        let new_x = if current_position.x > 1 { current_position.x - 1 } else { current_position.x };
                        let new_y = if current_position.y > 1 { current_position.y - 1 } else { current_position.y };
                        RobotAction::Move(Position::new(new_x, new_y))
                    } else {
                        RobotAction::Wait
                    }
                } else if energy > 5 {
                    // Alterner entre collecte et mouvement
                    if energy % 3 == 0 {
                        RobotAction::Collect
                    } else {
                        // Mouvement de recherche de ressources
                        let directions = [(1, 0), (0, 1), (-1, 0), (0, -1)];
                        let choice = (energy as usize / 3) % directions.len();
                        let (dx, dy) = directions[choice];

                        let new_x = (current_position.x as i32 + dx).max(0) as usize;
                        let new_y = (current_position.y as i32 + dy).max(0) as usize;

                        RobotAction::Move(Position::new(new_x, new_y))
                    }
                } else {
                    RobotAction::Wait
                }
            },
            crate::RobotBehavior::Scientifique => {
                if energy > 8 {
                    // Alterner entre analyse et mouvement vers zones d'intérêt
                    if energy % 2 == 0 {
                        RobotAction::Analyze
                    } else {
                        // Mouvement méthodique pour l'analyse
                        let new_x = if current_position.x < 20 { current_position.x + 1 } else { 1 };
                        let new_y = if new_x == 1 && current_position.y < 15 { current_position.y + 1 } else { current_position.y };

                        RobotAction::Move(Position::new(new_x, new_y))
                    }
                } else {
                    RobotAction::Wait
                }
            }
        }
    }
}