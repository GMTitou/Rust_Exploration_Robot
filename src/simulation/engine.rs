// src/simulation/engine.rs - Moteur de simulation principal
use crate::{Cell, Position};
use crate::robot::{Robot, RobotAction, BehaviorEngine};
use colored::Colorize;

pub struct SimulationEngine {
    pub map: Vec<Vec<Cell>>,
    pub robots: Vec<Robot>,
    pub turn: usize,
    pub width: usize,
    pub height: usize,
}

impl SimulationEngine {
    pub fn new(map: Vec<Vec<Cell>>, robots: Vec<Robot>) -> Self {
        let height = map.len();
        let width = if height > 0 { map[0].len() } else { 0 };

        SimulationEngine {
            map,
            robots,
            turn: 0,
            width,
            height,
        }
    }

    pub fn run(&mut self, max_turns: usize) {
        println!("{}", format!("🎮 Démarrage de la simulation ({} tours max)", max_turns).bright_green());

        for turn in 0..max_turns {
            self.turn = turn;
            self.execute_turn();

            if turn % 10 == 0 {
                self.print_status();
            }
        }

        println!("{}", "🏁 Simulation terminée !".bright_blue());
        self.print_final_report();
    }

    fn execute_turn(&mut self) {
        // Traiter chaque robot
        for i in 0..self.robots.len() {
            let robot = &self.robots[i];

            // Décider de l'action
            let action = BehaviorEngine::decide_action(
                robot.behavior,
                robot.position,
                robot.energy,
                robot.is_inventory_full(),
            );

            // Exécuter l'action
            self.execute_robot_action(i, action);
        }
    }

    fn execute_robot_action(&mut self, robot_index: usize, action: RobotAction) {
        match action {
            RobotAction::Move(new_position) => {
                if self.is_valid_position(new_position) &&
                    self.map[new_position.y][new_position.x].is_passable() {
                    self.robots[robot_index].move_to(new_position);
                }
            },
            RobotAction::Collect => {
                let robot = &mut self.robots[robot_index];
                let pos = robot.position;
                let cell = &mut self.map[pos.y][pos.x];

                // Collecter les ressources disponibles
                for (resource_type, &amount) in cell.resources.clone().iter() {
                    if amount > 0 {
                        let collected = robot.collect_resource(*resource_type, amount);
                        if collected > 0 {
                            *cell.resources.get_mut(resource_type).unwrap() -= collected;
                        }
                    }
                }
            },
            RobotAction::Analyze => {
                let robot = &mut self.robots[robot_index];
                let pos = robot.position;
                self.map[pos.y][pos.x].explored = true;
                robot.energy = robot.energy.saturating_sub(3);
            },
            RobotAction::Communicate(_targets) => {
                // TODO: Implémenter la communication
                self.robots[robot_index].energy = self.robots[robot_index].energy.saturating_sub(2);
            },
            RobotAction::Wait => {
                // Ne rien faire
            }
        }
    }

    fn is_valid_position(&self, pos: Position) -> bool {
        pos.x < self.width && pos.y < self.height
    }

    fn print_status(&self) {
        println!("\n{}", format!("📊 Tour {} - État de la simulation", self.turn).bright_cyan());

        for robot in &self.robots {
            let total_resources: u32 = robot.inventory.values().sum();
            println!(
                "🤖 Robot {} ({:?}): Position({},{}) Énergie:{} Ressources:{}",
                robot.id,
                robot.behavior,
                robot.position.x,
                robot.position.y,
                robot.energy,
                total_resources
            );
        }
    }

    fn print_final_report(&self) {
        println!("\n{}", "📋 RAPPORT FINAL".bright_yellow().bold());
        println!("{}", "================".bright_yellow());

        let mut total_energy_collected = 0;
        let mut total_minerals_collected = 0;
        let mut total_scientific_data = 0;

        for robot in &self.robots {
            total_energy_collected += robot.inventory.get(&crate::ResourceType::Energie).unwrap_or(&0);
            total_minerals_collected += robot.inventory.get(&crate::ResourceType::Mineraux).unwrap_or(&0);
            total_scientific_data += robot.inventory.get(&crate::ResourceType::LieuxInteret).unwrap_or(&0);
        }

        println!("⚡ Énergie collectée: {}", total_energy_collected);
        println!("🪨 Mineraux collectés: {}", total_minerals_collected);
        println!("🔬 Données scientifiques: {}", total_scientific_data);
        println!("🔄 Tours exécutés: {}", self.turn + 1);

        let explored_cells = self.map.iter()
            .flat_map(|row| row.iter())
            .filter(|cell| cell.explored)
            .count();

        let total_cells = self.width * self.height;
        let exploration_percentage = (explored_cells as f32 / total_cells as f32) * 100.0;

        println!("🗺️  Exploration: {:.1}% ({}/{})", exploration_percentage, explored_cells, total_cells);
    }
}