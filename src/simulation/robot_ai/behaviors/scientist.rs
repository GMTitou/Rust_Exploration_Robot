use crate::simulation::entities::{Map, Station};
use crate::simulation::robot_ai::behavior::RobotBehavior;
use crate::simulation::robot_ai::robot::Robot;
use crate::simulation::robot_ai::types::{AnalysisType, AnalyzeTask, Task, TaskType};
use crate::simulation::robot_ai::utils::SearchUtils;

pub struct ScientistBehavior;

impl RobotBehavior for ScientistBehavior {
    fn decide_next_action(&self, robot: &Robot, map: &Map, station: &Station) -> Option<Task> {
        if robot.carrying.is_some() || robot.is_low_energy() {
            return Some(Task {
                task_type: TaskType::ReturnToStation,
                target_position: Some((station.x, station.y)),
                priority: 9,
            });
        }

        if let Some(scientific_position) = self.find_scientific_interest(robot, map) {
            return Some(Task {
                task_type: TaskType::Analyze(AnalyzeTask {
                    target_position: scientific_position,
                    analysis_type: AnalysisType::Chemical,
                }),
                target_position: Some(scientific_position),
                priority: 8,
            });
        }

        if let Some(exploration_target) = self.find_systematic_exploration_target(robot, map) {
            return Some(Task {
                task_type: TaskType::Explore(crate::simulation::robot_ai::types::ExploreTask {
                    target_area: exploration_target,
                    radius: 3,
                }),
                target_position: Some(exploration_target),
                priority: 6,
            });
        }

        None
    }

    fn get_energy_consumption_rate(&self) -> u32 {
        4
    }

    fn get_max_energy(&self) -> u32 {
        100
    }

    fn get_low_energy_threshold(&self) -> u32 {
        25
    }
}

impl ScientistBehavior {
    fn find_scientific_interest(&self, robot: &Robot, map: &Map) -> Option<(usize, usize)> {
        SearchUtils::find_nearest_scientific_interest(robot.x, robot.y, 6, map)
    }

    fn find_systematic_exploration_target(
        &self,
        robot: &Robot,
        map: &Map,
    ) -> Option<(usize, usize)> {
        SearchUtils::find_nearest_unexplored(robot.x, robot.y, 6, map)
    }
}