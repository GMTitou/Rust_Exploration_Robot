use crate::simulation::entities::{Map, Station};
use crate::simulation::robot_ai::robot::Robot;
use crate::simulation::robot_ai::types::Task;

pub trait RobotBehavior {
    fn decide_next_action(&self, robot: &Robot, map: &Map, station: &Station) -> Option<Task>;
    fn get_energy_consumption_rate(&self) -> u32;
    fn get_max_energy(&self) -> u32;
    fn get_low_energy_threshold(&self) -> u32;
}

pub fn create_behavior(
    robot_type: &crate::simulation::robot_ai::types::RobotType,
) -> Box<dyn RobotBehavior> {
    match robot_type {
        crate::simulation::robot_ai::types::RobotType::Explorer => {
            Box::new(crate::simulation::robot_ai::behaviors::ExplorerBehavior)
        }
        crate::simulation::robot_ai::types::RobotType::Scientist => {
            Box::new(crate::simulation::robot_ai::behaviors::ScientistBehavior)
        }
    }
}