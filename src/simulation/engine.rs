// src/simulation/engine.rs - Moteur de simulation avec interface visuelle
use crate::{Cell, Position};
use crate::robot::{Robot, RobotAction, BehaviorEngine};
use colored::Colorize;
use std::io::{self, Write};
use std::thread;
use std::time::Duration;
use crate::display::DisplayEngine;

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

    /// Lance la simulation interactive
    pub fn run_interactive(&mut self) {
        println!("{}", "🎮 Mode interactif activé !".bright_green().bold());
        println!("{}", "Utilisez ENTER pour avancer tour par tour, 'q' pour quitter".bright_yellow());

        loop {
            self.display_current_states();
            self.display_controls();

            print!("\n> ");
            io::stdout().flush().unwrap();

            let mut input = String::new();
            io::stdin().read_line(&mut input).unwrap();
            let command = input.trim().to_lowercase();

            match command.as_str() {
                "q" | "quit" => {
                    println!("{}", "🏁 Simulation arrêtée par l'utilisateur".bright_red());
                    break;
                },
                "r" | "rapport" => {
                    self.print_detailed_report();
                },
                "s" | "auto" => {
                    self.run_auto_mode();
                },
                "" => {
                    // Exécuter un tour
                    self.execute_turn();
                    self.turn += 1;
                },
                _ => {
                    println!("{}", "Commande non reconnue. Utilisez ENTER, 'q', 'r', ou 's'".bright_red());
                }
            }
        }

        self.print_final_report();
    }

    /// Affiche l'état actuel de la simulation
    fn display_current_states(&self) {
        use crate::display::DisplayEngine;
        DisplayEngine::clear_screen();
        DisplayEngine::display_header(self.turn);
        DisplayEngine::display_map(&self.map, &self.robots);
        DisplayEngine::display_robot_stats(&self.robots);
        // Statistiques rapides
        let explored_cells = self.map.iter()
            .flat_map(|row| row.iter())
            .filter(|cell| cell.explored)
            .count();
        let total_cells = self.width * self.height;
        let exploration_percentage = (explored_cells as f32 / total_cells as f32) * 100.0;

        println!("\n📊 Exploration: {:.1}% ({}/{})", exploration_percentage, explored_cells, total_cells);
    }

    /// Affiche les contrôles
    fn display_controls(&self) {
        use crate::display::DisplayEngine;
        DisplayEngine::display_controls();
    }

    /// Mode automatique rapide
    pub fn run_auto_mode(&mut self) {
        println!("{}", "🚀 Mode automatique lancé ! (Ctrl+C pour arrêter)".bright_green().bold());

        loop {
            self.display_current_states();
            self.execute_turn();
            self.turn += 1;

            // Pause entre les tours
            thread::sleep(Duration::from_millis(100)); // 10 tours/seconde

            // Arrêter après 1000 tours pour éviter une boucle infinie
            if self.turn >= 1000 {
                println!("{}", "⚠️  Limite de 1000 tours atteinte - arrêt automatique".bright_yellow());
                break;
            }
        }
    }

    /// Affiche l'état actuel de la simulation
    fn display_current_state(&self) {
        DisplayEngine::clear_screen();
        DisplayEngine::display_header(self.turn);
        DisplayEngine::display_map(&self.map, &self.robots);
        DisplayEngine::display_robot_stats(&self.robots);

        // Statistiques rapides
        let explored_cells = self.map.iter()
            .flat_map(|row| row.iter())
            .filter(|cell| cell.explored)
            .count();
        let total_cells = self.width * self.height;
        let exploration_percentage = (explored_cells as f32 / total_cells as f32) * 100.0;

        println!("\n📊 Exploration: {:.1}% ({}/{})", exploration_percentage, explored_cells, total_cells);
    }

    /// Lance la simulation classique (sans interface)
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
                    // Libérer l'ancienne position
                    let old_pos = self.robots[robot_index].position;
                    self.map[old_pos.y][old_pos.x].occupied_by = None;

                    // Déplacer le robot
                    self.robots[robot_index].move_to(new_position);

                    // Occuper la nouvelle position
                    self.map[new_position.y][new_position.x].occupied_by = Some(robot_index);
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
                // Ne rien faire, mais récupérer un peu d'énergie
                let robot = &mut self.robots[robot_index];
                robot.energy = (robot.energy + 1).min(100);
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

    fn print_detailed_report(&self) {
        DisplayEngine::clear_screen();
        println!("{}", "📋 RAPPORT DÉTAILLÉ".bright_yellow().bold());
        println!("{}", "====================".bright_yellow());

        // Statistiques globales
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
        println!("🔄 Tours exécutés: {}", self.turn);

        let explored_cells = self.map.iter()
            .flat_map(|row| row.iter())
            .filter(|cell| cell.explored)
            .count();

        let total_cells = self.width * self.height;
        let exploration_percentage = (explored_cells as f32 / total_cells as f32) * 100.0;

        println!("🗺️  Exploration: {:.1}% ({}/{})", exploration_percentage, explored_cells, total_cells);

        // Détail par robot
        println!("\n{}", "📋 Détail par robot:".bright_cyan());
        DisplayEngine::display_robot_stats(&self.robots);

        println!("\n{}", "Appuyez sur ENTER pour continuer...".bright_white());
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
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
        println!("🔄 Tours exécutés: {}", self.turn);

        let explored_cells = self.map.iter()
            .flat_map(|row| row.iter())
            .filter(|cell| cell.explored)
            .count();

        let total_cells = self.width * self.height;
        let exploration_percentage = (explored_cells as f32 / total_cells as f32) * 100.0;

        println!("🗺️  Exploration: {:.1}% ({}/{})", exploration_percentage, explored_cells, total_cells);

        println!("\n{}", "🎯 Merci d'avoir utilisé EREEA !".bright_green().bold());
    }
}