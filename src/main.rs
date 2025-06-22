// src/main.rs - Point d'entrée du projet EREEA
mod map;
mod robot;
mod simulation;
mod utils;

// Import des types depuis lib.rs
use final_project::{Position, TerrainType, Cell, ResourceType, RobotBehavior};
use map::MapGenerator;
use robot::Robot;
use simulation::SimulationEngine;
use utils::NoiseGenerator;
use colored::Colorize;

fn main() {
    // Bannière de démarrage
    println!("{}", "🚀 EREEA - Essaim de Robots pour l'Exploration Astrobiologique".bright_blue().bold());
    println!("{}", "   Projet Rust - Simulation de robots autonomes".bright_cyan());
    println!();

    // Test des utilitaires de génération
    println!("{}", "=== Test des utilitaires ===".bright_yellow());
    let noise_gen = NoiseGenerator::new(42, 0.1);
    let test_noise = noise_gen.get_noise(5.0, 10.0);
    let test_terrain = noise_gen.get_terrain(5.0, 10.0);
    println!("🌊 Valeur de bruit: {:.3}", test_noise);
    println!("🗺️  Terrain généré: {:?}", test_terrain);

    // Génération de la carte
    println!("\n{}", "=== Génération de la carte ===".bright_yellow());
    let map_generator = MapGenerator::new(42, 20, 15);
    let game_map = map_generator.generate();
    println!("✅ Carte générée: {}x{}", game_map[0].len(), game_map.len());

    // Création des robots
    println!("\n{}", "=== Création des robots ===".bright_yellow());
    let mut robots = Vec::new();

    robots.push(Robot::new(0, Position::new(1, 1), RobotBehavior::Explorateur));
    robots.push(Robot::new(1, Position::new(2, 1), RobotBehavior::Collecteur));
    robots.push(Robot::new(2, Position::new(3, 1), RobotBehavior::Scientifique));

    for robot in &robots {
        println!("🤖 Robot {} créé: {:?} à ({},{})",
                 robot.id, robot.behavior, robot.position.x, robot.position.y);
    }

    // Test des types de base
    println!("\n{}", "=== Tests des types de base ===".bright_yellow());
    let pos = Position::new(5, 10);
    let mut cell = Cell::new(TerrainType::Plaine);
    cell.add_resource(ResourceType::Energie, 50);
    cell.add_resource(ResourceType::Mineraux, 25);

    println!("📍 Position de test: {:?}", pos);
    println!("🗺️  Cellule créée: terrain = {:?}", cell.terrain);
    println!("⚡ Ressources: {:?}", cell.resources);

    // Vérification des modules
    println!("\n{}", "=== Modules chargés ===".bright_cyan());
    println!("✅ 📍 Map - Génération de cartes");
    println!("✅ 🤖 Robot - Gestion des robots");
    println!("✅ ⚙️  Simulation - Moteur de simulation");
    println!("✅ 🔧 Utils - Utilitaires et génération procédurale");

    // Lancement de la simulation
    println!("\n{}", "=== Lancement de la simulation ===".bright_green());
    let mut simulation = SimulationEngine::new(game_map, robots);
    simulation.run(500); 

    println!("\n{}", "🎯 Simulation terminée ! Projet EREEA opérationnel.".bright_green().bold());
}