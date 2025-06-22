use crate::{Cell, TerrainType, ResourceType, Position};
use crate::robot::Robot;
use crossterm::{
    event::{self, Event, KeyCode, KeyEventKind},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    cursor,
    style::{Color, Print, ResetColor, SetForegroundColor, SetBackgroundColor},
};
use std::collections::HashMap;
use std::io::{self, Write, stdout};
use std::time::{Duration, Instant};

pub struct GuiEngine {
    width: usize,
    height: usize,
    last_update: Instant,
    paused: bool,
    speed: u64, // millisecondes entre les updates
}

impl GuiEngine {
    pub fn new(width: usize, height: usize) -> Self {
        GuiEngine {
            width,
            height,
            last_update: Instant::now(),
            paused: false,
            speed: 3000, // 3 secondes par défaut au lieu de 0.5
        }
    }

    pub fn run_gui_simulation(&mut self, mut simulation: crate::simulation::SimulationEngine) -> io::Result<()> {
        // Initialiser le terminal
        enable_raw_mode()?;
        execute!(stdout(), EnterAlternateScreen)?;

        let mut turn = 0;
        let mut auto_mode = true;

        loop {
            // Gestion des événements clavier
            if event::poll(Duration::from_millis(50))? {
                if let Event::Key(key) = event::read()? {
                    if key.kind == KeyEventKind::Press {
                        match key.code {
                            KeyCode::Char('q') | KeyCode::Esc => break,
                            KeyCode::Char(' ') => {
                                self.paused = !self.paused;
                            },
                            KeyCode::Char('+') | KeyCode::Up => {
                                if self.speed > 100 {
                                    self.speed -= 200; // Réduire par pas de 200ms
                                }
                            },
                            KeyCode::Char('-') | KeyCode::Down => {
                                if self.speed < 10000 {
                                    self.speed += 200; // Augmenter par pas de 200ms
                                }
                            },
                            KeyCode::Enter => {
                                if self.paused {
                                    simulation.execute_gui_turn();
                                    turn += 1;
                                }
                            },
                            KeyCode::Char('a') => {
                                auto_mode = !auto_mode;
                                self.paused = !auto_mode;
                            },
                            _ => {}
                        }
                    }
                }
            }

            // Mettre à jour la simulation si ce n'est pas en pause
            if !self.paused && self.last_update.elapsed() >= Duration::from_millis(self.speed) {
                simulation.execute_gui_turn();
                turn += 1;
                self.last_update = Instant::now();
            }

            // Dessiner l'interface
            self.draw_interface(&simulation, turn, auto_mode)?;
        }

        // Nettoyer le terminal
        execute!(stdout(), LeaveAlternateScreen)?;
        disable_raw_mode()?;

        Ok(())
    }

    fn draw_interface(&self, simulation: &crate::simulation::SimulationEngine, turn: usize, auto_mode: bool) -> io::Result<()> {
        // Effacer l'écran
        execute!(stdout(), crossterm::terminal::Clear(crossterm::terminal::ClearType::All))?;
        execute!(stdout(), cursor::MoveTo(0, 0))?;

        // En-tête avec bordure
        self.draw_header(turn, auto_mode)?;

        // Carte avec bordure
        self.draw_map(&simulation.map, &simulation.robots)?;

        // Statistiques des robots
        self.draw_robot_stats(&simulation.robots)?;

        // Contrôles
        self.draw_controls()?;

        stdout().flush()?;
        Ok(())
    }

    fn draw_header(&self, turn: usize, auto_mode: bool) -> io::Result<()> {
        let border = "╔".to_string() + &"═".repeat(78) + "╗";
        execute!(stdout(), SetForegroundColor(Color::Cyan), Print(&border), ResetColor)?;
        execute!(stdout(), cursor::MoveToNextLine(1))?;

        let title = format!("║{:^78}║", "🚀 EREEA - SIMULATION TEMPS RÉEL 🚀");
        execute!(stdout(), SetForegroundColor(Color::Cyan), Print(&title), ResetColor)?;
        execute!(stdout(), cursor::MoveToNextLine(1))?;

        let status = format!("║ Tour: {:6} │ Mode: {:10} │ Vitesse: {:3}ms │ État: {:8} ║",
                             turn,
                             if auto_mode { "AUTO" } else { "MANUEL" },
                             self.speed,
                             if self.paused { "PAUSE" } else { "ACTIF" });
        execute!(stdout(), SetForegroundColor(Color::Cyan), Print(&status), ResetColor)?;
        execute!(stdout(), cursor::MoveToNextLine(1))?;

        let bottom_border = "╚".to_string() + &"═".repeat(78) + "╝";
        execute!(stdout(), SetForegroundColor(Color::Cyan), Print(&bottom_border), ResetColor)?;
        execute!(stdout(), cursor::MoveToNextLine(1))?;

        Ok(())
    }

    fn draw_map(&self, map: &Vec<Vec<Cell>>, robots: &Vec<Robot>) -> io::Result<()> {
        let height = map.len();
        let width = if height > 0 { map[0].len() } else { 0 };

        // Créer une map des positions des robots
        let mut robot_positions: HashMap<Position, &Robot> = HashMap::new();
        for robot in robots {
            robot_positions.insert(robot.position, robot);
        }

        // En-tête simplifié avec coordonnées tous les 10
        execute!(stdout(), Print("     "))?;
        for x in (0..width).step_by(10) {
            execute!(stdout(), SetForegroundColor(Color::DarkGrey), Print(&format!("{:<10}", x)), ResetColor)?;
        }
        execute!(stdout(), cursor::MoveToNextLine(1))?;

        // Bordure supérieure
        execute!(stdout(), Print("     "))?;
        let top_border = "┌".to_string() + &"─".repeat(width) + "┐";
        execute!(stdout(), SetForegroundColor(Color::White), Print(&top_border), ResetColor)?;
        execute!(stdout(), cursor::MoveToNextLine(1))?;

        // Afficher la carte ligne par ligne
        for (y, row) in map.iter().enumerate() {
            // Numéro de ligne tous les 5
            if y % 5 == 0 {
                execute!(stdout(), SetForegroundColor(Color::DarkGrey), Print(&format!("{:3} ", y)), ResetColor)?;
            } else {
                execute!(stdout(), Print("    "))?;
            }
            execute!(stdout(), SetForegroundColor(Color::White), Print("│"), ResetColor)?;

            for (x, cell) in row.iter().enumerate() {
                let pos = Position::new(x, y);

                // Priorité 1: Robots (toujours visibles)
                if let Some(robot) = robot_positions.get(&pos) {
                    let (bg_color, symbol) = match robot.behavior {
                        crate::RobotBehavior::Explorateur => (Color::Green, "E"),
                        crate::RobotBehavior::Collecteur => (Color::Yellow, "C"),
                        crate::RobotBehavior::Scientifique => (Color::Blue, "S"),
                    };
                    execute!(stdout(), 
                        SetBackgroundColor(bg_color),
                        SetForegroundColor(Color::Black),
                        Print(symbol),
                        ResetColor
                    )?;
                    continue;
                }

                // Priorité 2: Ressources importantes seulement
                if !cell.resources.is_empty() {
                    if cell.resources.contains_key(&ResourceType::LieuxInteret) {
                        execute!(stdout(), SetForegroundColor(Color::Magenta), Print("*"), ResetColor)?;
                        continue;
                    }
                    if cell.resources.get(&ResourceType::Energie).unwrap_or(&0) > &30 {
                        execute!(stdout(), SetForegroundColor(Color::Cyan), Print("E"), ResetColor)?;
                        continue;
                    }
                    if cell.resources.get(&ResourceType::Mineraux).unwrap_or(&0) > &40 {
                        execute!(stdout(), SetForegroundColor(Color::Red), Print("M"), ResetColor)?;
                        continue;
                    }
                }

                // Priorité 3: Terrain simplifié
                match cell.terrain {
                    TerrainType::Obstacle => {
                        execute!(stdout(), SetForegroundColor(Color::Red), Print("#"), ResetColor)?;
                    },
                    TerrainType::Montagne => {
                        execute!(stdout(), SetForegroundColor(Color::DarkGrey), Print("^"), ResetColor)?;
                    },
                    TerrainType::Cratere => {
                        execute!(stdout(), SetForegroundColor(Color::Yellow), Print("o"), ResetColor)?;
                    },
                    TerrainType::Plaine => {
                        if cell.explored {
                            execute!(stdout(), SetForegroundColor(Color::DarkGrey), Print("·"), ResetColor)?;
                        } else {
                            execute!(stdout(), Print(" "))?; // Espace vide pour simplifier
                        }
                    },
                }
            }

            execute!(stdout(), SetForegroundColor(Color::White), Print("│"), ResetColor)?;
            execute!(stdout(), cursor::MoveToNextLine(1))?;
        }

        // Bordure inférieure
        execute!(stdout(), Print("     "))?;
        let bottom_border = "└".to_string() + &"─".repeat(width) + "┘";
        execute!(stdout(), SetForegroundColor(Color::White), Print(&bottom_border), ResetColor)?;
        execute!(stdout(), cursor::MoveToNextLine(1))?;

        Ok(())
    }

    fn draw_robot_stats(&self, robots: &Vec<Robot>) -> io::Result<()> {
        execute!(stdout(), cursor::MoveToNextLine(1))?;
        execute!(stdout(), SetForegroundColor(Color::Cyan), Print("═══ ÉTAT DES ROBOTS ═══"), ResetColor)?;
        execute!(stdout(), cursor::MoveToNextLine(1))?;

        for (i, robot) in robots.iter().enumerate() {
            if i >= 8 { break; } // Limiter l'affichage

            let total_resources: u32 = robot.inventory.values().sum();
            let energy_bar = self.create_energy_bar(robot.energy);

            let (color, behavior_str) = match robot.behavior {
                crate::RobotBehavior::Explorateur => (Color::Green, "EXP"),
                crate::RobotBehavior::Collecteur => (Color::Yellow, "COL"),
                crate::RobotBehavior::Scientifique => (Color::Blue, "SCI"),
            };

            let stats = format!("Robot {} [{}] Pos({:2},{:2}) {} Res:{}",
                                robot.id, behavior_str, robot.position.x, robot.position.y, energy_bar, total_resources);

            execute!(stdout(), SetForegroundColor(color), Print(&stats), ResetColor)?;
            execute!(stdout(), cursor::MoveToNextLine(1))?;
        }

        Ok(())
    }

    fn create_energy_bar(&self, energy: u32) -> String {
        let bar_length = 10;
        let filled = (energy * bar_length / 100).min(bar_length);
        let empty = bar_length - filled;

        format!("[{}{}] {:3}/100",
                "█".repeat(filled as usize),
                "░".repeat(empty as usize),
                energy)
    }

    fn draw_controls(&self) -> io::Result<()> {
        execute!(stdout(), cursor::MoveToNextLine(1))?;
        execute!(stdout(), SetForegroundColor(Color::Yellow), Print("═══ LÉGENDE ═══"), ResetColor)?;
        execute!(stdout(), cursor::MoveToNextLine(1))?;

        // Légende des robots
        execute!(stdout(), 
            SetBackgroundColor(Color::Green), SetForegroundColor(Color::Black), Print("E"), ResetColor,
            Print(" Explorateur  "),
            SetBackgroundColor(Color::Yellow), SetForegroundColor(Color::Black), Print("C"), ResetColor,
            Print(" Collecteur  "),
            SetBackgroundColor(Color::Blue), SetForegroundColor(Color::White), Print("S"), ResetColor,
            Print(" Scientifique")
        )?;
        execute!(stdout(), cursor::MoveToNextLine(1))?;

        // Légende simplifiée des éléments
        execute!(stdout(), 
            SetForegroundColor(Color::Red), Print("#"), ResetColor, Print(" Obstacle  "),
            SetForegroundColor(Color::DarkGrey), Print("^"), ResetColor, Print(" Montagne  "),
            SetForegroundColor(Color::Yellow), Print("o"), ResetColor, Print(" Cratère  "),
            SetForegroundColor(Color::DarkGrey), Print("·"), ResetColor, Print(" Exploré")
        )?;
        execute!(stdout(), cursor::MoveToNextLine(1))?;

        // Ressources importantes uniquement
        execute!(stdout(), 
            SetForegroundColor(Color::Cyan), Print("E"), ResetColor, Print(" Énergie(>30)  "),
            SetForegroundColor(Color::Red), Print("M"), ResetColor, Print(" Mineraux(>40)  "),
            SetForegroundColor(Color::Magenta), Print("*"), ResetColor, Print(" Site scientifique")
        )?;
        execute!(stdout(), cursor::MoveToNextLine(2))?;

        execute!(stdout(), SetForegroundColor(Color::Cyan), Print("═══ CONTRÔLES ═══"), ResetColor)?;
        execute!(stdout(), cursor::MoveToNextLine(1))?;

        let controls = [
            "[ESPACE] Pause/Play",
            "[1-5] Vitesse: 1s/2s/3s/5s/0.5s",
            "[+/-] Ajuster vitesse",
            "[A] Mode Auto/Manuel",
            "[ENTER] Tour suivant (pause)",
            "[Q/ESC] Quitter"
        ];

        for control in &controls {
            execute!(stdout(), SetForegroundColor(Color::White), Print(control), ResetColor)?;
            execute!(stdout(), cursor::MoveToNextLine(1))?;
        }

        Ok(())
    }
}

// Extension pour SimulationEngine
impl crate::simulation::SimulationEngine {
    pub fn execute_gui_turn(&mut self) {
        // Traiter chaque robot
        for i in 0..self.robots.len() {
            let robot = &self.robots[i];

            // Décider de l'action
            let action = crate::robot::BehaviorEngine::decide_action(
                robot.behavior,
                robot.position,
                robot.energy,
                robot.is_inventory_full(),
            );

            // Exécuter l'action
            self.execute_gui_robot_action(i, action);
        }

        self.turn += 1;
    }

    fn execute_gui_robot_action(&mut self, robot_index: usize, action: crate::robot::RobotAction) {
        match action {
            crate::robot::RobotAction::Move(new_position) => {
                if self.is_gui_valid_position(new_position) &&
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
            crate::robot::RobotAction::Collect => {
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
            crate::robot::RobotAction::Analyze => {
                let robot = &mut self.robots[robot_index];
                let pos = robot.position;
                self.map[pos.y][pos.x].explored = true;
                robot.energy = robot.energy.saturating_sub(3);
            },
            crate::robot::RobotAction::Communicate(_targets) => {
                self.robots[robot_index].energy = self.robots[robot_index].energy.saturating_sub(2);
            },
            crate::robot::RobotAction::Wait => {
                // Récupérer un peu d'énergie
                let robot = &mut self.robots[robot_index];
                robot.energy = (robot.energy + 1).min(100);
            }
        }
    }

    fn is_gui_valid_position(&self, pos: Position) -> bool {
        pos.x < self.width && pos.y < self.height
    }
}