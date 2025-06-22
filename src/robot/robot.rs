// src/robot/robot.rs - Structure principale des robots
use crate::{Position, ResourceType};
use std::collections::HashMap;
use final_project::{RobotBehavior, RobotModule};

#[derive(Debug, Clone)]
pub struct Robot {
    pub id: usize,
    pub position: Position,
    pub behavior: RobotBehavior,
    pub modules: Vec<RobotModule>,
    pub energy: u32,
    pub inventory: HashMap<ResourceType, u32>,
    pub max_inventory: u32,
    pub communication_range: usize,
}

impl Robot {
    pub fn new(id: usize, position: Position, behavior: RobotBehavior) -> Self {
        let modules = Self::default_modules_for_behavior(behavior);

        Robot {
            id,
            position,
            behavior,
            modules,
            energy: 100,
            inventory: HashMap::new(),
            max_inventory: 50,
            communication_range: 5,
        }
    }

    fn default_modules_for_behavior(behavior: RobotBehavior) -> Vec<RobotModule> {
        match behavior {
            RobotBehavior::Explorateur => vec![
                RobotModule::Deplacement,
                RobotModule::Communication,
                RobotModule::ImageHauteResolution,
            ],
            RobotBehavior::Collecteur => vec![
                RobotModule::Deplacement,
                RobotModule::CollecteEnergie,
                RobotModule::CollecteMineraux,
            ],
            RobotBehavior::Scientifique => vec![
                RobotModule::AnalyseChimique,
                RobotModule::CollecteDonnees,
                RobotModule::Communication,
            ],
        }
    }

    pub fn has_module(&self, module: RobotModule) -> bool {
        self.modules.contains(&module)
    }

    pub fn can_collect(&self, resource_type: ResourceType) -> bool {
        match resource_type {
            ResourceType::Energie => self.has_module(RobotModule::CollecteEnergie),
            ResourceType::Mineraux => self.has_module(RobotModule::CollecteMineraux),
            ResourceType::LieuxInteret => self.has_module(RobotModule::CollecteDonnees),
        }
    }

    pub fn collect_resource(&mut self, resource_type: ResourceType, amount: u32) -> u32 {
        if !self.can_collect(resource_type) {
            return 0;
        }

        let current_total: u32 = self.inventory.values().sum();
        let available_space = self.max_inventory.saturating_sub(current_total);
        let collected = amount.min(available_space);

        if collected > 0 {
            *self.inventory.entry(resource_type).or_insert(0) += collected;
        }

        collected
    }

    pub fn move_to(&mut self, new_position: Position) -> bool {
        if self.energy > 0 {
            self.position = new_position;
            self.energy = self.energy.saturating_sub(1);
            true
        } else {
            false
        }
    }

    pub fn is_inventory_full(&self) -> bool {
        let current_total: u32 = self.inventory.values().sum();
        current_total >= self.max_inventory
    }
}