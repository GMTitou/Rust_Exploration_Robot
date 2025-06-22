// src/display.rs - Module d'affichage de la simulation
use crate::{Cell, TerrainType, ResourceType, Position};
use crate::robot::Robot;
use colored::Colorize;
use std::collections::HashMap;

pub struct DisplayEngine;

impl DisplayEngine {
    /// Affiche la carte complète avec robots et ressources
    pub fn display_map(map: &Vec<Vec<Cell>>, robots: &Vec<Robot>) {
        let height = map.len();
        let width = if height > 0 { map[0].len() } else { 0 };

        // Créer une map des positions des robots
        let mut robot_positions: HashMap<Position, &Robot> = HashMap::new();
        for robot in robots {
            robot_positions.insert(robot.position, robot);
        }

        // Afficher l'en-tête
        print!("   ");
        for x in 0..width {
            print!("{:2}", x % 10);
        }
        println!();

        // Afficher chaque ligne
        for (y, row) in map.iter().enumerate() {
            print!("{:2} ", y);

            for (x, cell) in row.iter().enumerate() {
                let pos = Position::new(x, y);

                // Vérifier s'il y a un robot à cette position
                if let Some(robot) = robot_positions.get(&pos) {
                    let robot_symbol = Self::get_robot_symbol(robot);
                    print!("{}", robot_symbol);
                } else {
                    let cell_symbol = Self::get_cell_symbol(cell);
                    print!("{}", cell_symbol);
                }
            }
            println!();
        }

        // Afficher la légende
        Self::display_legend();
    }

    /// Obtient le symbole d'un robot avec couleur
    fn get_robot_symbol(robot: &Robot) -> String {
        match robot.behavior {
            crate::RobotBehavior::Explorateur => {
                format!("E{}", robot.id).bright_green().to_string()
            },
            crate::RobotBehavior::Collecteur => {
                format!("C{}", robot.id).bright_yellow().to_string()
            },
            crate::RobotBehavior::Scientifique => {
                format!("S{}", robot.id).bright_blue().to_string()
            },
        }
    }

    /// Obtient le symbole d'une cellule avec couleur
    fn get_cell_symbol(cell: &Cell) -> String {
        // Priorité : ressources > terrain exploré > terrain normal
        if !cell.resources.is_empty() {
            // Afficher la ressource la plus importante
            if cell.resources.contains_key(&ResourceType::LieuxInteret) {
                "🔬".bright_magenta().to_string()
            } else if cell.resources.contains_key(&ResourceType::Energie) {
                "⚡".bright_cyan().to_string()
            } else if cell.resources.contains_key(&ResourceType::Mineraux) {
                "💎".bright_red().to_string()
            } else {
                Self::get_terrain_symbol(&cell.terrain, cell.explored)
            }
        } else {
            Self::get_terrain_symbol(&cell.terrain, cell.explored)
        }
    }

    /// Obtient le symbole du terrain
    fn get_terrain_symbol(terrain: &TerrainType, explored: bool) -> String {
        let symbol = match terrain {
            TerrainType::Plaine => if explored { "·" } else { "." },
            TerrainType::Montagne => "^",
            TerrainType::Cratere => "O",
            TerrainType::Obstacle => "#",
        };

        let colored_symbol = match terrain {
            TerrainType::Plaine => {
                if explored {
                    symbol.bright_white().to_string()
                } else {
                    symbol.white().to_string()
                }
            },
            TerrainType::Montagne => symbol.bright_black().to_string(),
            TerrainType::Cratere => symbol.yellow().to_string(),
            TerrainType::Obstacle => symbol.red().to_string(),
        };

        colored_symbol
    }

    /// Affiche la légende
    fn display_legend() {
        println!("\n{}", "=== LÉGENDE ===".bright_cyan().bold());
        println!("🤖 Robots:");
        println!("  {} - Explorateur", "E#".bright_green());
        println!("  {} - Collecteur", "C#".bright_yellow());
        println!("  {} - Scientifique", "S#".bright_blue());

        println!("\n🗺️  Terrains:");
        println!("  {} - Plaine", ".".white());
        println!("  {} - Plaine explorée", "·".bright_white());
        println!("  {} - Montagne", "^".bright_black());
        println!("  {} - Cratère", "O".yellow());
        println!("  {} - Obstacle", "#".red());

        println!("\n⚡ Ressources:");
        println!("  {} - Énergie", "⚡".bright_cyan());
        println!("  {} - Mineraux", "💎".bright_red());
        println!("  {} - Lieux d'intérêt", "🔬".bright_magenta());
    }

    /// Affiche les statistiques des robots
    pub fn display_robot_stats(robots: &Vec<Robot>) {
        println!("\n{}", "=== ÉTAT DES ROBOTS ===".bright_cyan().bold());

        for robot in robots {
            let total_resources: u32 = robot.inventory.values().sum();
            let energy_bar = Self::create_energy_bar(robot.energy);
            let behavior_color = match robot.behavior {
                crate::RobotBehavior::Explorateur => "Explorateur".bright_green(),
                crate::RobotBehavior::Collecteur => "Collecteur".bright_yellow(),
                crate::RobotBehavior::Scientifique => "Scientifique".bright_blue(),
            };

            println!(
                "🤖 Robot {} ({}) - Pos({},{}) {} - Ressources: {}",
                robot.id,
                behavior_color,
                robot.position.x,
                robot.position.y,
                energy_bar,
                total_resources
            );

            // Détail des ressources si le robot en a
            if total_resources > 0 {
                print!("   💼 ");
                for (resource_type, amount) in &robot.inventory {
                    let resource_symbol = match resource_type {
                        ResourceType::Energie => "⚡",
                        ResourceType::Mineraux => "💎",
                        ResourceType::LieuxInteret => "🔬",
                    };
                    print!("{}{} ", resource_symbol, amount);
                }
                println!();
            }
        }
    }

    /// Crée une barre d'énergie visuelle
    fn create_energy_bar(energy: u32) -> String {
        let max_energy = 100;
        let bar_length = 10;
        let filled = (energy * bar_length / max_energy).min(bar_length);
        let empty = bar_length - filled;

        let filled_bar = "█".repeat(filled as usize);
        let empty_bar = "░".repeat(empty as usize);

        let colored_bar = if energy > 70 {
            format!("{}{}",filled_bar.bright_green(), empty_bar.white())
        } else if energy > 30 {
            format!("{}{}",filled_bar.yellow(), empty_bar.white())
        } else {
            format!("{}{}",filled_bar.red(), empty_bar.white())
        };

        format!("Énergie: {} [{}] {}/100",
                if energy > 70 { "⚡".green() } else if energy > 30 { "⚡".yellow() } else { "⚡".red() },
                colored_bar,
                energy)
    }

    /// Efface l'écran (compatible Windows/Linux)
    pub fn clear_screen() {
        print!("\x1B[2J\x1B[1;1H");
    }

    /// Affiche l'en-tête de la simulation
    pub fn display_header(turn: usize) {
        println!("{}", "╔═══════════════════════════════════════════════════════════╗".bright_blue());
        println!("{}", "║           🚀 EREEA - SIMULATION EN TEMPS RÉEL            ║".bright_blue());
        println!("{}", format!("║                        Tour: {:6}                      ║", turn).bright_blue());
        println!("{}", "╚═══════════════════════════════════════════════════════════╝".bright_blue());
    }

    /// Affiche les contrôles
    pub fn display_controls() {
        println!("\n{}", "⌨️  CONTRÔLES".bright_yellow().bold());
        println!("  [ENTER] - Tour suivant");
        println!("  'q' + [ENTER] - Quitter");
        println!("  'r' + [ENTER] - Rapport détaillé");
        println!("  's' + [ENTER] - Mode automatique (10 tours/seconde)");
    }
}