// src/main.rs - Point d'entrée du projet EREEA avec interface
mod map;
mod robot;
mod simulation;
mod utils;
mod display;
mod gui;

// Import des types depuis lib.rs - CORRIGÉ pour final_project
use final_project::{Position, TerrainType, Cell, ResourceType, RobotBehavior};
use map::MapGenerator;
use robot::Robot;
use simulation::SimulationEngine;
use utils::NoiseGenerator;
use gui::GuiEngine;
use colored::Colorize;
use std::io::{self, Write};

fn main() {
    // Bannière de démarrage
    println!("{}", "🚀 EREEA - Essaim de Robots pour l'Exploration Astrobiologique".bright_blue().bold());
    println!("{}", "   Projet Rust - Simulation de robots autonomes".bright_cyan());
    println!();

    // Menu de sélection du mode
    loop {
        println!("{}", "=== SÉLECTION DU MODE ===".bright_yellow().bold());
        println!("1. 🎮 Mode interactif (console)");
        println!("2. 🖥️  Mode GUI temps réel (recommandé)");
        println!("3. 🚀 Mode automatique rapide");
        println!("4. 📊 Mode classique (50 tours)");
        println!("5. 🧪 Tests et démonstration");
        println!("6. ❌ Quitter");

        print!("\nChoisissez un mode (1-6): ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        match input.trim() {
            "1" => {
                launch_interactive_mode();
                break;
            },
            "2" => {
                launch_gui_mode();
                break;
            },
            "3" => {
                launch_auto_mode();
                break;
            },
            "4" => {
                launch_classic_mode();
                break;
            },
            "5" => {
                run_demo();
                break;
            },
            "6" => {
                println!("{}", "👋 Au revoir !".bright_green());
                break;
            },
            _ => {
                println!("{}", "❌ Choix invalide. Entrez un numéro entre 1 et 6.".bright_red());
            }
        }
    }
}

fn launch_gui_mode() {
    println!("{}", "🖥️  Mode GUI temps réel sélectionné !".bright_green().bold());
    println!("{}", "Lancement de l'interface graphique...".bright_cyan());

    // Génération de la carte et des robots
    let (game_map, robots) = create_simulation_environment();
    let simulation = SimulationEngine::new(game_map, robots);

    // Créer l'interface GUI
    let mut gui = GuiEngine::new(simulation.width, simulation.height);

    // Lancer la simulation GUI
    match gui.run_gui_simulation(simulation) {
        Ok(_) => println!("{}", "✅ Simulation GUI terminée avec succès !".bright_green()),
        Err(e) => println!("{}", format!("❌ Erreur GUI: {}", e).bright_red()),
    }
}

fn launch_interactive_mode() {
    println!("{}", "🎮 Mode interactif sélectionné !".bright_green().bold());

    // Génération de la carte et des robots
    let (game_map, robots) = create_simulation_environment();

    // Lancer la simulation interactive
    let mut simulation = SimulationEngine::new(game_map, robots);
    simulation.run_interactive();
}

fn launch_auto_mode() {
    println!("{}", "🚀 Mode automatique sélectionné !".bright_green().bold());

    // Génération de la carte et des robots
    let (game_map, robots) = create_simulation_environment();

    // Lancer la simulation automatique
    let mut simulation = SimulationEngine::new(game_map, robots);
    simulation.run_auto_mode();
}

fn launch_classic_mode() {
    println!("{}", "📊 Mode classique sélectionné !".bright_green().bold());

    // Test des utilitaires de génération
    println!("{}", "=== Test des utilitaires ===".bright_yellow());
    let noise_gen = NoiseGenerator::new(42, 0.1);
    let test_noise = noise_gen.get_noise(5.0, 10.0);
    let test_terrain = noise_gen.get_terrain(5.0, 10.0);
    println!("🌊 Valeur de bruit: {:.3}", test_noise);
    println!("🗺️  Terrain généré: {:?}", test_terrain);

    // Génération de la carte et des robots
    let (game_map, robots) = create_simulation_environment();

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
    println!("✅ 🎨 Display - Interface visuelle");

    // Lancement de la simulation classique
    println!("\n{}", "=== Lancement de la simulation ===".bright_green());
    let mut simulation = SimulationEngine::new(game_map, robots);
    simulation.run(50); // 50 tours de simulation

    println!("\n{}", "🎯 Simulation terminée ! Projet EREEA opérationnel.".bright_green().bold());
}

fn run_demo() {
    println!("{}", "🧪 Mode démonstration lancé !".bright_green().bold());

    // Créer une petite carte pour la démo
    let mut demo_map = Vec::new();
    for y in 0..10 {
        let mut row = Vec::new();
        for x in 0..15 {
            let terrain = match (x, y) {
                (0, _) | (14, _) | (_, 0) | (_, 9) => TerrainType::Obstacle, // Bordures
                (3, 3) | (7, 6) | (11, 2) => TerrainType::Montagne,
                (5, 7) | (9, 4) => TerrainType::Cratere,
                _ => TerrainType::Plaine,
            };

            let mut cell = Cell::new(terrain);

            // Ajouter quelques ressources pour la démo
            match (x, y) {
                (4, 4) => cell.add_resource(ResourceType::Energie, 30),
                (8, 3) => cell.add_resource(ResourceType::Mineraux, 50),
                (5, 7) => cell.add_resource(ResourceType::LieuxInteret, 1),
                (9, 4) => cell.add_resource(ResourceType::LieuxInteret, 1),
                _ => {}
            }

            row.push(cell);
        }
        demo_map.push(row);
    }

    // Créer des robots pour la démo
    let demo_robots = vec![
        Robot::new(1, Position::new(2, 2), RobotBehavior::Explorateur),
        Robot::new(2, Position::new(3, 2), RobotBehavior::Collecteur),
        Robot::new(3, Position::new(4, 2), RobotBehavior::Scientifique),
    ];

    println!("\n{}", "🗺️  Carte de démonstration générée !".bright_cyan());
    println!("{}", "Cette carte contient des ressources pré-placées pour démontrer le système.".bright_white());

    // Afficher la carte initiale
    use crate::display::DisplayEngine;
    DisplayEngine::display_map(&demo_map, &demo_robots);
    DisplayEngine::display_robot_stats(&demo_robots);

    println!("\n{}", "Appuyez sur ENTER pour lancer la simulation interactive...".bright_yellow());
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    // Lancer la simulation
    let mut simulation = SimulationEngine::new(demo_map, demo_robots);
    simulation.run_interactive();
}

fn create_simulation_environment() -> (Vec<Vec<Cell>>, Vec<Robot>) {
    println!("{}", "=== Génération de l'environnement ===".bright_yellow());

    // Carte plus petite et moins dense pour la lisibilité
    let map_generator = MapGenerator::new(42, 30, 15); // 30x15 au lieu de 50x30
    let game_map = map_generator.generate();
    println!("✅ Carte générée: {}x{}", game_map[0].len(), game_map.len());

    // Moins de robots pour plus de clarté
    let mut robots = Vec::new();

    // 3 explorateurs
    robots.push(Robot::new(1, Position::new(2, 2), RobotBehavior::Explorateur));
    robots.push(Robot::new(2, Position::new(8, 3), RobotBehavior::Explorateur));
    robots.push(Robot::new(3, Position::new(15, 5), RobotBehavior::Explorateur));

    // 2 collecteurs
    robots.push(Robot::new(4, Position::new(5, 7), RobotBehavior::Collecteur));
    robots.push(Robot::new(5, Position::new(12, 4), RobotBehavior::Collecteur));

    // 1 scientifique
    robots.push(Robot::new(6, Position::new(18, 6), RobotBehavior::Scientifique));

    println!("✅ {} robots créés avec différents comportements", robots.len());

    for robot in &robots {
        println!("🤖 Robot {} ({:?}) déployé à ({:2},{:2})",
                 robot.id, robot.behavior, robot.position.x, robot.position.y);
    }

    (game_map, robots)
}2