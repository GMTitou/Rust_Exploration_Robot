use crate::{Map, Robot};

pub struct Display;

impl Display {
    pub fn clear_screen() {
        print!("\x1B[2J\x1B[1;1H");
    }

    pub fn show_header() {
        println!("╔══════════════════════════════════════════════════════════════╗");
        println!("║                  EREEA - SIMULATION SPATIALE                 ║");
        println!("║          Essaim de Robots pour l'Exploration et              ║");
        println!("║               l'Étude Astrobiologique                        ║");
        println!("╚══════════════════════════════════════════════════════════════╝");
        println!();
    }

    pub fn show_legend() {
        println!("📋 LÉGENDE:");
        println!("  E = Explorateur  A = Analyseur  H = Collecteur  S = Éclaireur");
        println!("  # = Roche  M = Minéraux  🔋 = Énergie  ! = Point d'intérêt");
        println!("  X = Obstacle  · = Zone explorée    = Zone inexplorée");
        println!();
    }

    pub fn show_robot_status(robots: &[Robot]) {
        println!("🤖 ÉTAT DES ROBOTS:");
        for robot in robots {
            let status_icon = if robot.energy > 50 { "🟢" }
            else if robot.energy > 20 { "🟡" }
            else { "🔴" };

            println!("  {} Robot {} ({}) - Position: ({},{}) - Énergie: {}/{} - Inventaire: {} objets",
                     status_icon,
                     robot.id,
                     robot.robot_type.name(),
                     robot.x,
                     robot.y,
                     robot.energy,
                     robot.max_energy,
                     robot.inventory.len()
            );
        }
        println!();
    }

    pub fn show_statistics(engine: &crate::simulation::SimulationEngine) {
        let stats = engine.get_statistics();
        println!("📊 STATISTIQUES DE MISSION:");
        println!("  Zones explorées: {} cellules", stats.total_exploration);
        println!("  Étape actuelle: {}", engine.step_count);

        let total_efficiency: f32 = stats.robot_efficiency.values().sum();
        let avg_efficiency = if !stats.robot_efficiency.is_empty() {
            total_efficiency / stats.robot_efficiency.len() as f32 * 100.0
        } else {
            0.0
        };

        println!("  Efficacité moyenne: {:.1}%", avg_efficiency);
        println!();
    }
}