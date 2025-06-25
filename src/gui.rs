use crate::{Map, Robot, SimulationEngine};
use std::io::{self, Write};

pub struct GameUI {
    pub paused: bool,
    pub speed: u64,
    pub auto_mode: bool,
}

impl GameUI {
    pub fn new() -> Self {
        GameUI {
            paused: false,
            speed: 500,
            auto_mode: true,
        }
    }

    pub fn show_main_menu(&self) {
        println!("🎮 CONTRÔLES DE SIMULATION:");
        println!("  [ESPACE] Pause/Reprendre");
        println!("  [+/-] Ajuster la vitesse");
        println!("  [A] Mode automatique on/off");
        println!("  [R] Redémarrer la simulation");
        println!("  [Q] Quitter");
        println!("  [H] Afficher l'aide");
        println!();
    }

    pub fn handle_input(&mut self) -> Option<char> {
        // Simulation d'entrée utilisateur (dans un vrai projet, utiliser crossterm)
        None
    }

    pub fn render_full_interface(&self, engine: &SimulationEngine, robots: &[Robot]) {
        crate::display::Display::clear_screen();
        crate::display::Display::show_header();

        // Afficher la carte avec robots
        engine.display_with_robots(robots);

        // Afficher les informations
        crate::display::Display::show_legend();
        crate::display::Display::show_robot_status(robots);
        crate::display::Display::show_statistics(engine);

        // Afficher les contrôles
        self.show_main_menu();

        // Afficher l'état de la simulation
        let status = if self.paused { "⏸️ PAUSE" } else { "▶️ EN COURS" };
        println!("État: {} | Vitesse: {}ms | Mode: {}",
                 status,
                 self.speed,
                 if self.auto_mode { "Auto" } else { "Manuel" }
        );
    }
}
