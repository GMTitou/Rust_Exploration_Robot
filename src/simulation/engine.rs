use crate::{Map, Robot};
use std::collections::HashMap;

pub struct SimulationEngine {
    pub map: Map,
    pub step_count: u32,
    pub statistics: SimulationStats,
}

#[derive(Debug, Default)]
pub struct SimulationStats {
    pub total_exploration: u32,
    pub resources_collected: HashMap<String, u32>,
    pub robot_efficiency: HashMap<u32, f32>,
}

impl SimulationEngine {
    pub fn new(map: Map) -> Self {
        SimulationEngine {
            map,
            step_count: 0,
            statistics: SimulationStats::default(),
        }
    }

    pub fn step(&mut self, robots: &mut [Robot]) {
        self.step_count += 1;

        // Mettre à jour chaque robot
        for robot in robots.iter_mut() {
            if robot.is_operational() {
                robot.update(&mut self.map);
            }
        }

        // Mettre à jour les statistiques
        self.update_statistics(robots);
    }

    pub fn display_with_robots(&self, robots: &[Robot]) {
        self.map.display_with_robots(robots);
    }

    fn update_statistics(&mut self, robots: &[Robot]) {
        // Calculer l'exploration totale
        let mut explored_cells = 0;
        for row in &self.map.cells {
            for cell in row {
                if cell.explored {
                    explored_cells += 1;
                }
            }
        }
        self.statistics.total_exploration = explored_cells;

        // Calculer l'efficacité des robots
        for robot in robots {
            let efficiency = if robot.max_energy > 0 {
                robot.energy as f32 / robot.max_energy as f32
            } else {
                0.0
            };
            self.statistics.robot_efficiency.insert(robot.id, efficiency);
        }
    }

    pub fn get_statistics(&self) -> &SimulationStats {
        &self.statistics
    }
}