use crate::simulation::entities::{Map, Station};
use crate::simulation::robot_ai::behavior::RobotBehavior;
use crate::simulation::robot_ai::robot::Robot;
use crate::simulation::robot_ai::types::{ExploreTask, Task, TaskType};
use crate::simulation::robot_ai::utils::SearchUtils;

pub struct ExplorerBehavior;

impl RobotBehavior for ExplorerBehavior {
    fn decide_next_action(&self, robot: &Robot, map: &Map, station: &Station) -> Option<Task> {
        if robot.carrying.is_some() || robot.is_low_energy() {
            return Some(Task {
                task_type: TaskType::ReturnToStation,
                target_position: Some((station.x, station.y)),
                priority: 9,
            });
        }

        if let Some(unexplored_area) = self.find_unexplored_area(robot, map) {
            return Some(Task {
                task_type: TaskType::Explore(ExploreTask {
                    target_area: unexplored_area,
                    radius: 3,
                }),
                target_position: Some(unexplored_area),
                priority: 7,
            });
        }

        if let Some(random_target) = self.get_random_exploration_target(robot, map) {
            return Some(Task {
                task_type: TaskType::Explore(ExploreTask {
                    target_area: random_target,
                    radius: 2,
                }),
                target_position: Some(random_target),
                priority: 5,
            });
        }

        None
    }

    fn get_energy_consumption_rate(&self) -> u32 {
        2
    }

    fn get_max_energy(&self) -> u32 {
        100
    }

    fn get_low_energy_threshold(&self) -> u32 {
        20
    }
}

impl ExplorerBehavior {
    fn find_unexplored_area(&self, robot: &Robot, map: &Map) -> Option<(usize, usize)> {
        SearchUtils::find_nearest_unexplored(robot.x, robot.y, 5, map)
    }

    fn get_random_exploration_target(&self, robot: &Robot, map: &Map) -> Option<(usize, usize)> {
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};

        let mut hasher = DefaultHasher::new();
        robot.id.hash(&mut hasher);
        robot.x.hash(&mut hasher);
        robot.y.hash(&mut hasher);
        let seed = hasher.finish();

        let target_x = ((seed % map.width as u64) as usize)
            .max(1)
            .min(map.width - 2);
        let target_y = (((seed >> 16) % map.height as u64) as usize)
            .max(1)
            .min(map.height - 2);

        if map.terrain[target_y][target_x] == 0 {
            Some((target_x, target_y))
        } else {
            None
        }
    }
}