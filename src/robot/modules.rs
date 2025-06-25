use super::robot::RobotType;

#[derive(Debug, Clone)]
pub struct RobotModule {
    pub name: String,
    pub energy_cost: u32,
    pub efficiency: f32,
}

impl RobotModule {
    pub fn new(name: String, energy_cost: u32, efficiency: f32) -> Self {
        RobotModule {
            name,
            energy_cost,
            efficiency,
        }
    }

    pub fn get_modules_for_type(robot_type: &RobotType) -> Vec<RobotModule> {
        match robot_type {
            RobotType::Explorer => vec![
                RobotModule::new("Cartographie".to_string(), 3, 0.8),
                RobotModule::new("Communication".to_string(), 2, 0.9),
                RobotModule::new("Navigation".to_string(), 1, 0.95),
            ],
            RobotType::Analyzer => vec![
                RobotModule::new("Spectromètre".to_string(), 8, 0.9),
                RobotModule::new("Microscope".to_string(), 5, 0.85),
                RobotModule::new("Chimie".to_string(), 6, 0.8),
            ],
            RobotType::Harvester => vec![
                RobotModule::new("Forage".to_string(), 10, 0.75),
                RobotModule::new("Stockage".to_string(), 2, 0.95),
                RobotModule::new("Raffinage".to_string(), 7, 0.7),
            ],
            RobotType::Scout => vec![
                RobotModule::new("Vitesse".to_string(), 4, 0.9),
                RobotModule::new("Détection".to_string(), 3, 0.85),
                RobotModule::new("Stealth".to_string(), 5, 0.8),
            ],
        }
    }

    pub fn activate(&self, current_energy: u32) -> Option<u32> {
        if current_energy >= self.energy_cost {
            Some(current_energy - self.energy_cost)
        } else {
            None
        }
    }

    pub fn get_efficiency_bonus(&self) -> f32 {
        self.efficiency
    }

    pub fn is_compatible_with(&self, robot_type: &RobotType) -> bool {
        let modules = Self::get_modules_for_type(robot_type);
        modules.iter().any(|m| m.name == self.name)
    }
}