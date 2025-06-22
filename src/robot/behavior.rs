// src/robot/behavior.rs - Comportements des robots
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
                if energy < 20 {
                    RobotAction::Wait
                } else {
                    // Logique d'exploration simple
                    let new_x = current_position.x + 1;
                    let new_y = current_position.y;
                    RobotAction::Move(Position::new(new_x, new_y))
                }
            },
            crate::RobotBehavior::Collecteur => {
                if inventory_full {
                    RobotAction::Wait // Devrait retourner à la base
                } else if energy > 10 {
                    RobotAction::Collect
                } else {
                    RobotAction::Wait
                }
            },
            crate::RobotBehavior::Scientifique => {
                if energy > 15 {
                    RobotAction::Analyze
                } else {
                    RobotAction::Wait
                }
            }
        }
    }
}