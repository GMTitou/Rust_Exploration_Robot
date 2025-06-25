use crate::simulation::entities::{Map, ResourceType, Station};
use crate::simulation::robot_ai::behavior::RobotBehavior;
use crate::simulation::robot_ai::robot::Robot;
use crate::simulation::robot_ai::types::{HarvestTask, Task, TaskType};
use crate::simulation::robot_ai::utils::SearchUtils;

pub struct HarvesterBehavior;

impl RobotBehavior for HarvesterBehavior {
    fn decide_next_action(&self, robot: &Robot, map: &Map, station: &Station) -> Option<Task> {
        if robot.carrying.is_some() || robot.is_low_energy() {
            return Some(Task {
                task_type: TaskType::ReturnToStation,
                target_position: Some((station.x, station.y)),
                priority: 10,
            });
        }

        if let Some((resource_type, position)) = self.find_preferred_resource(robot, map) {
            return Some(Task {
                task_type: TaskType::Harvest(HarvestTask {
                    resource_type,
                    target_position: position,
                }),
                target_position: Some(position),
                priority: 8,
            });
        }

        if let Some(exploration_target) = self.find_exploration_target(robot, map) {
            return Some(Task {
                task_type: TaskType::Explore(crate::simulation::robot_ai::types::ExploreTask {
                    target_area: exploration_target,
                    radius: 2,
                }),
                target_position: Some(exploration_target),
                priority: 6,
            });
        }

        None
    }

    fn get_energy_consumption_rate(&self) -> u32 {
        3
    }

    fn get_max_energy(&self) -> u32 {
        100
    }

    fn get_low_energy_threshold(&self) -> u32 {
        15
    }
}

impl HarvesterBehavior {
    fn find_preferred_resource(
        &self,
        robot: &Robot,
        map: &Map,
    ) -> Option<(ResourceType, (usize, usize))> {
        let preferred_types = vec![ResourceType::Energy, ResourceType::Mineral];

        for resource_type in preferred_types {
            if let Some((position, found_type)) = SearchUtils::find_nearest_resource(
                robot.x,
                robot.y,
                4,
                map,
                &[resource_type.clone()],
            ) {
                return Some((found_type, position));
            }
        }

        None
    }

    fn find_exploration_target(&self, robot: &Robot, map: &Map) -> Option<(usize, usize)> {
        SearchUtils::find_nearest_unexplored(robot.x, robot.y, 4, map)
    }
}