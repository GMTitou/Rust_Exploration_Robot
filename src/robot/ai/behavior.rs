use crate::maps::entities::{Map, Station};
use crate::robot::ai::tasks::Task;
use crate::robot::core::robot::Robot;
use crate::robot::core::types::RobotType;

pub trait RobotBehavior {
    fn decide_next_action(&self, robot: &Robot, map: &Map, station: &Station) -> Option<Task>;
    fn get_energy_consumption_rate(&self) -> u32;
    fn get_max_energy(&self) -> u32;
    fn get_low_energy_threshold(&self) -> u32;
}

pub fn create_behavior(robot_type: &RobotType) -> Box<dyn RobotBehavior> {
    match robot_type {
        RobotType::Explorer => {
            Box::new(crate::robot::ai::behaviors::ExplorerBehavior)
        }
        RobotType::Scientist => {
            Box::new(crate::robot::ai::behaviors::ScientistBehavior)
        }
    }
}