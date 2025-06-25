use final_project::{Map, Robot, RobotType, SimulationEngine};
use macroquad::prelude::*;

struct GameState {
    engine: SimulationEngine,
    robots: Vec<Robot>,
    step: u32,
    running: bool,
    speed: f32,
    last_update: f64,
    cell_size: f32,
    map_offset_x: f32,
    map_offset_y: f32,
    selected_robot: Option<usize>,
    show_energy: bool,
}

impl GameState {
    fn new() -> Self {
        // Créer une carte plus adaptée à l'écran
        let mut map = Map::new(70, 40);
        map.generate_terrain();

        let robots = vec![
            Robot::new(1, RobotType::Explorer, 10, 10),
            Robot::new(2, RobotType::Analyzer, 20, 15),
            Robot::new(3, RobotType::Harvester, 30, 20),
            Robot::new(4, RobotType::Scout, 40, 25),
            Robot::new(5, RobotType::Explorer, 50, 30),
            Robot::new(6, RobotType::Harvester, 60, 35),
        ];

        let engine = SimulationEngine::new(map);

        Self {
            engine,
            robots,
            step: 0,
            running: true,
            speed: 3.0,
            last_update: get_time(),
            cell_size: 10.0, // Taille plus grande des cellules
            map_offset_x: 0.0, // Sera calculé pour centrer
            map_offset_y: 0.0, // Sera calculé pour centrer
            selected_robot: None,
            show_energy: true,
        }
    }

    fn update(&mut self) {
        // Calculer l'offset pour centrer la carte
        let map_width = self.engine.map.width as f32 * self.cell_size;
        let map_height = self.engine.map.height as f32 * self.cell_size;
        let panel_width = 350.0;
        let available_width = screen_width() - panel_width - 40.0;

        self.map_offset_x = (available_width - map_width) / 2.0 + 20.0;
        self.map_offset_y = (screen_height() - map_height) / 2.0 + 50.0;

        // Contrôles
        if is_key_pressed(KeyCode::Space) {
            self.running = !self.running;
        }

        if is_key_pressed(KeyCode::Equal) || is_key_pressed(KeyCode::KpAdd) {
            self.speed = (self.speed + 0.5).min(10.0);
        }

        if is_key_pressed(KeyCode::Minus) || is_key_pressed(KeyCode::KpSubtract) {
            self.speed = (self.speed - 0.5).max(0.1);
        }

        if is_key_pressed(KeyCode::E) {
            self.show_energy = !self.show_energy;
        }

        // Zoom avec molette
        let (_x, wheel_y) = mouse_wheel();
        if wheel_y != 0.0 {
            self.cell_size = (self.cell_size + wheel_y * 0.5).clamp(6.0, 20.0);
        }

        // Sélection de robot
        if is_mouse_button_pressed(MouseButton::Left) {
            let (mouse_x, mouse_y) = mouse_position();
            if mouse_x >= self.map_offset_x && mouse_y >= self.map_offset_y {
                let map_x = ((mouse_x - self.map_offset_x) / self.cell_size) as usize;
                let map_y = ((mouse_y - self.map_offset_y) / self.cell_size) as usize;

                self.selected_robot = self.robots.iter().position(|robot|
                    robot.x == map_x && robot.y == map_y
                );
            }
        }

        // Mise à jour de la simulation
        if self.running {
            let now = get_time();
            if now - self.last_update >= 1.0 / self.speed as f64 {
                self.step += 1;

                for robot in &mut self.robots {
                    if robot.is_operational() {
                        robot.update(&mut self.engine.map);
                    }
                }

                self.last_update = now;
            }
        }
    }

    fn draw(&self) {
        clear_background(Color::from_rgba(15, 15, 25, 255));

        // Titre principal
        draw_text("🚀 EREEA - Simulation d'Exploration Spatiale", 30.0, 35.0, 28.0, WHITE);

        // Barre de statut
        let status = if self.running { "▶ EN COURS" } else { "⏸ PAUSE" };
        let status_color = if self.running { GREEN } else { YELLOW };

        draw_text(&format!("Étape: {} | {} | Vitesse: {:.1}/s | Zoom: {:.1}x",
                           self.step, status, self.speed, self.cell_size / 10.0),
                  30.0, 65.0, 20.0, status_color);

        draw_text("Contrôles: [ESPACE] Pause | [+/-] Vitesse | [E] Énergie | [Molette] Zoom | [Clic] Sélection",
                  30.0, 85.0, 16.0, LIGHTGRAY);

        // Bordure autour de la carte
        let border_thickness = 3.0;
        let map_width = self.engine.map.width as f32 * self.cell_size;
        let map_height = self.engine.map.height as f32 * self.cell_size;

        draw_rectangle_lines(
            self.map_offset_x - border_thickness,
            self.map_offset_y - border_thickness,
            map_width + border_thickness * 2.0,
            map_height + border_thickness * 2.0,
            border_thickness,
            WHITE
        );

        // Dessiner la carte
        for y in 0..self.engine.map.height {
            for x in 0..self.engine.map.width {
                let cell = &self.engine.map.cells[y][x];
                let screen_x = self.map_offset_x + x as f32 * self.cell_size;
                let screen_y = self.map_offset_y + y as f32 * self.cell_size;

                let color = match cell.terrain {
                    final_project::map::TerrainType::Empty => {
                        if cell.explored {
                            Color::from_rgba(70, 70, 80, 255)
                        } else {
                            Color::from_rgba(25, 25, 35, 255)
                        }
                    },
                    final_project::map::TerrainType::Rock => Color::from_rgba(120, 120, 120, 255),
                    final_project::map::TerrainType::Mineral => Color::from_rgba(100, 200, 255, 255),
                    final_project::map::TerrainType::Energy => Color::from_rgba(255, 100, 100, 255),
                    final_project::map::TerrainType::Interest => Color::from_rgba(255, 255, 100, 255),
                    final_project::map::TerrainType::Obstacle => Color::from_rgba(80, 40, 20, 255),
                };

                draw_rectangle(screen_x, screen_y, self.cell_size, self.cell_size, color);

                // Grille subtile
                if self.cell_size > 8.0 {
                    draw_rectangle_lines(screen_x, screen_y, self.cell_size, self.cell_size,
                                         0.5, Color::from_rgba(50, 50, 50, 100));
                }
            }
        }

        // Dessiner les robots
        for (i, robot) in self.robots.iter().enumerate() {
            let screen_x = self.map_offset_x + robot.x as f32 * self.cell_size + self.cell_size / 2.0;
            let screen_y = self.map_offset_y + robot.y as f32 * self.cell_size + self.cell_size / 2.0;

            let robot_color = match robot.robot_type {
                RobotType::Explorer => Color::from_rgba(50, 255, 50, 255),
                RobotType::Analyzer => Color::from_rgba(50, 150, 255, 255),
                RobotType::Harvester => Color::from_rgba(255, 180, 50, 255),
                RobotType::Scout => Color::from_rgba(255, 50, 255, 255),
            };

            let robot_radius = if Some(i) == self.selected_robot {
                self.cell_size * 0.4
            } else {
                self.cell_size * 0.35
            };

            // Robot principal
            draw_circle(screen_x, screen_y, robot_radius, robot_color);

            // Bordure pour robot sélectionné
            if Some(i) == self.selected_robot {
                draw_circle_lines(screen_x, screen_y, robot_radius + 2.0, 3.0, WHITE);

                // Rayon de communication
                draw_circle_lines(screen_x, screen_y,
                                  robot.communication_range as f32 * self.cell_size,
                                  1.0, Color::from_rgba(255, 255, 255, 80));
            }

            // ID du robot
            let font_size = (self.cell_size * 0.7).max(12.0);
            let text = robot.id.to_string();
            let text_size = measure_text(&text, None, font_size as u16, 1.0);
            draw_text(&text,
                      screen_x - text_size.width / 2.0,
                      screen_y + text_size.height / 4.0,
                      font_size, WHITE);

            // Barre d'énergie
            if self.show_energy && robot.is_operational() {
                let energy_ratio = robot.energy as f32 / robot.max_energy as f32;
                let bar_width = self.cell_size * 0.8;
                let bar_height = (self.cell_size * 0.1).max(3.0);
                let bar_x = screen_x - bar_width / 2.0;
                let bar_y = screen_y - self.cell_size / 2.0 - 12.0;

                // Fond de la barre
                draw_rectangle(bar_x, bar_y, bar_width, bar_height, DARKGRAY);

                // Barre d'énergie
                let energy_color = if energy_ratio > 0.6 { GREEN }
                else if energy_ratio > 0.3 { YELLOW }
                else { RED };

                draw_rectangle(bar_x, bar_y, bar_width * energy_ratio, bar_height, energy_color);

                // Bordure de la barre
                draw_rectangle_lines(bar_x, bar_y, bar_width, bar_height, 1.0, WHITE);
            }
        }

        // Panneau d'informations (à droite)
        let panel_width = 350.0;
        let panel_x = screen_width() - panel_width;
        let panel_height = screen_height() - 20.0;

        // Fond du panneau
        draw_rectangle(panel_x, 10.0, panel_width, panel_height, Color::from_rgba(20, 20, 30, 200));
        draw_rectangle_lines(panel_x, 10.0, panel_width, panel_height, 2.0, WHITE);

        let mut y_pos = 40.0;

        // Titre du panneau
        draw_text("📊 TABLEAU DE BORD", panel_x + 15.0, y_pos, 22.0, YELLOW);
        y_pos += 40.0;

        // Statistiques générales
        draw_rectangle(panel_x + 10.0, y_pos - 5.0, panel_width - 20.0, 80.0, Color::from_rgba(30, 30, 40, 150));
        draw_text("STATISTIQUES", panel_x + 15.0, y_pos + 15.0, 18.0, SKYBLUE);
        y_pos += 35.0;

        let stats = self.engine.get_statistics();
        draw_text(&format!("🗺️ Zones explorées: {}", stats.total_exploration),
                  panel_x + 15.0, y_pos, 16.0, WHITE);
        y_pos += 25.0;

        let operational = self.robots.iter().filter(|r| r.is_operational()).count();
        draw_text(&format!("🤖 Robots actifs: {}/{}", operational, self.robots.len()),
                  panel_x + 15.0, y_pos, 16.0, WHITE);
        y_pos += 40.0;

        // Section robots
        draw_rectangle(panel_x + 10.0, y_pos - 5.0, panel_width - 20.0, 200.0, Color::from_rgba(30, 30, 40, 150));
        draw_text("🤖 ROBOTS", panel_x + 15.0, y_pos + 15.0, 18.0, SKYBLUE);
        y_pos += 35.0;

        for (i, robot) in self.robots.iter().enumerate() {
            let is_selected = Some(i) == self.selected_robot;
            let type_color = match robot.robot_type {
                RobotType::Explorer => GREEN,
                RobotType::Analyzer => BLUE,
                RobotType::Harvester => ORANGE,
                RobotType::Scout => MAGENTA,
            };

            let text_color = if is_selected { YELLOW } else { type_color };
            let prefix = if is_selected { "► " } else { "  " };

            draw_text(&format!("{}Robot {} ({})", prefix, robot.id, robot.robot_type.name()),
                      panel_x + 15.0, y_pos, 15.0, text_color);
            y_pos += 20.0;

            if is_selected {
                draw_text(&format!("   Pos: ({}, {}) | État: {:?}",
                                   robot.x, robot.y, robot.behavior.get_current_state()),
                          panel_x + 20.0, y_pos, 13.0, LIGHTGRAY);
                y_pos += 16.0;

                draw_text(&format!("   Énergie: {}/{} | Inventaire: {}",
                                   robot.energy, robot.max_energy, robot.inventory.len()),
                          panel_x + 20.0, y_pos, 13.0, LIGHTGRAY);
                y_pos += 20.0;
            }
        }

        // Légende
        y_pos = screen_height() - 180.0;
        draw_rectangle(panel_x + 10.0, y_pos - 5.0, panel_width - 20.0, 160.0, Color::from_rgba(30, 30, 40, 150));
        draw_text("🗺️ LÉGENDE", panel_x + 15.0, y_pos + 15.0, 18.0, SKYBLUE);
        y_pos += 35.0;

        let legend = [
            ("💎 Minéraux (M)", Color::from_rgba(100, 200, 255, 255)),
            ("⚡ Énergie (E)", Color::from_rgba(255, 100, 100, 255)),
            ("⭐ Intérêt (!)", Color::from_rgba(255, 255, 100, 255)),
            ("🪨 Roche (#)", Color::from_rgba(120, 120, 120, 255)),
            ("👁️ Exploré (·)", Color::from_rgba(70, 70, 80, 255)),
            ("🚫 Obstacle (X)", Color::from_rgba(80, 40, 20, 255)),
        ];

        for (name, color) in &legend {
            draw_rectangle(panel_x + 15.0, y_pos - 8.0, 15.0, 15.0, *color);
            draw_rectangle_lines(panel_x + 15.0, y_pos - 8.0, 15.0, 15.0, 1.0, WHITE);
            draw_text(name, panel_x + 40.0, y_pos, 14.0, WHITE);
            y_pos += 22.0;
        }
    }
}

#[macroquad::main("EREEA Simulation")]
async fn main() {
    request_new_screen_size(1600.0, 1000.0);

    let mut game = GameState::new();

    loop {
        game.update();
        game.draw();
        next_frame().await;
    }
}