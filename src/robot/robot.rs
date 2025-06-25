use super::behavior::RobotBehavior;
use crate::map::Map;

#[derive(Debug, Clone)]
pub enum RobotType {
    Explorer,   // Exploration générale
    Analyzer,   // Analyse chimique et géologique
    Harvester,  // Collecte de ressources
    Scout,      // Reconnaissance rapide
}

impl RobotType {
    pub fn name(&self) -> &str {
        match self {
            RobotType::Explorer => "Explorateur",
            RobotType::Analyzer => "Analyseur",
            RobotType::Harvester => "Collecteur",
            RobotType::Scout => "Éclaireur",
        }
    }

    pub fn max_energy(&self) -> u32 {
        match self {
            RobotType::Explorer => 100,
            RobotType::Analyzer => 80,
            RobotType::Harvester => 120,
            RobotType::Scout => 60,
        }
    }

    pub fn speed(&self) -> u32 {
        match self {
            RobotType::Explorer => 2,
            RobotType::Analyzer => 1,
            RobotType::Harvester => 1,
            RobotType::Scout => 3,
        }
    }
}

#[derive(Debug)]
pub struct Robot {
    pub id: u32,
    pub robot_type: RobotType,
    pub x: usize,
    pub y: usize,
    pub energy: u32,
    pub max_energy: u32,
    pub inventory: Vec<String>,
    pub behavior: RobotBehavior,
    pub communication_range: u32,
    pub discovered_locations: Vec<(usize, usize)>,
}

impl Robot {
    pub fn new(id: u32, robot_type: RobotType, x: usize, y: usize) -> Self {
        let max_energy = robot_type.max_energy();
        Robot {
            id,
            robot_type: robot_type.clone(),
            x,
            y,
            energy: max_energy,
            max_energy,
            inventory: Vec::new(),
            behavior: RobotBehavior::new(robot_type),
            communication_range: 5,
            discovered_locations: Vec::new(),
        }
    }

    pub fn update(&mut self, map: &mut Map) {
        // Marquer la position actuelle comme explorée
        map.mark_explored(self.x, self.y);

        // Extraire les données nécessaires pour éviter les emprunts multiples
        let robot_id = self.id;
        let robot_type = self.robot_type.clone();
        let mut current_x = self.x;
        let mut current_y = self.y;
        let mut current_energy = self.energy;

        // Mettre à jour le comportement avec les données extraites
        self.behavior.update_robot_state(&mut current_x, &mut current_y, &mut current_energy, map, &robot_type, robot_id);

        // Appliquer les changements
        self.x = current_x;
        self.y = current_y;
        self.energy = current_energy;

        // Consommer de l'énergie
        self.consume_energy(1);

        // Mettre à jour la position sur la carte
        map.update_robot_position(self.id, self.x, self.y);
    }

    pub fn move_to(&mut self, new_x: usize, new_y: usize, map: &Map) -> bool {
        if map.is_passable(new_x, new_y) && self.energy > 2 {
            self.x = new_x;
            self.y = new_y;
            self.consume_energy(2);
            true
        } else {
            false
        }
    }

    pub fn consume_energy(&mut self, amount: u32) {
        self.energy = self.energy.saturating_sub(amount);
    }

    pub fn add_resource(&mut self, resource: String) {
        self.inventory.push(resource);
    }

    pub fn get_symbol(&self) -> char {
        match self.robot_type {
            RobotType::Explorer => 'E',
            RobotType::Analyzer => 'A',
            RobotType::Harvester => 'H',
            RobotType::Scout => 'S',
        }
    }

    pub fn is_operational(&self) -> bool {
        self.energy > 0
    }

    pub fn scan_area(&self, map: &Map, range: usize) -> Vec<(usize, usize, String)> {
        let mut discoveries = Vec::new();

        for dy in -(range as i32)..=(range as i32) {
            for dx in -(range as i32)..=(range as i32) {
                let scan_x = (self.x as i32 + dx).max(0) as usize;
                let scan_y = (self.y as i32 + dy).max(0) as usize;

                if let Some(cell) = map.get_cell(scan_x, scan_y) {
                    if cell.terrain.exploration_value() > 0 {
                        discoveries.push((scan_x, scan_y, format!("{:?}", cell.terrain)));
                    }
                }
            }
        }

        discoveries
    }
}