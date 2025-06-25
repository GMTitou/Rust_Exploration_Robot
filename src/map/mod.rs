pub mod generator;
pub mod terrain;

use crate::utils::noise::NoiseGenerator;
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq)]
pub enum TerrainType {
    Empty,      // ' '
    Rock,       // '#'
    Mineral,    // 'M'
    Energy,     // 'E'
    Interest,   // '!'
    Obstacle,   // 'X'
}

#[derive(Debug, Clone)]
pub struct Cell {
    pub terrain: TerrainType,
    pub energy_level: u32,
    pub mineral_count: u32,
    pub explored: bool,
    pub danger_level: u8,
    pub depleted: bool, // Nouvelle propriété pour les ressources épuisées
}

impl Cell {
    pub fn new(terrain: TerrainType) -> Self {
        Cell {
            terrain: terrain.clone(),
            energy_level: match terrain {
                TerrainType::Energy => 100,
                TerrainType::Mineral => 50,
                _ => 10,
            },
            mineral_count: match terrain {
                TerrainType::Mineral => 5,
                _ => 0,
            },
            explored: false,
            danger_level: 0,
            depleted: false,
        }
    }
}

pub struct Map {
    pub width: usize,
    pub height: usize,
    pub cells: Vec<Vec<Cell>>,
    pub robot_positions: HashMap<u32, (usize, usize)>,
    noise_gen: NoiseGenerator,
}

impl Map {
    pub fn new(width: usize, height: usize) -> Self {
        let cells = vec![vec![Cell::new(TerrainType::Empty); width]; height];
        Map {
            width,
            height,
            cells,
            robot_positions: HashMap::new(),
            noise_gen: NoiseGenerator::new(),
        }
    }

    pub fn generate_terrain(&mut self) {
        // Génération avec moins de ressources énergétiques
        for y in 0..self.height {
            for x in 0..self.width {
                let noise_val = self.noise_gen.perlin_2d(x as f64 * 0.08, y as f64 * 0.08);

                // Réduire drastiquement les ressources énergétiques
                if noise_val > 0.5 {
                    self.cells[y][x] = Cell::new(TerrainType::Rock);
                } else if noise_val > 0.25 {
                    self.cells[y][x] = Cell::new(TerrainType::Mineral);
                } else if noise_val < -0.6 { // Seuil plus bas pour moins d'énergie
                    self.cells[y][x] = Cell::new(TerrainType::Energy);
                } else if noise_val < -0.7 { // Seuil encore plus bas
                    self.cells[y][x] = Cell::new(TerrainType::Interest);
                }
            }
        }

        // Ajouter quelques obstacles stratégiques
        self.add_obstacles();
    }

    fn add_obstacles(&mut self) {
        // Bordures
        for x in 0..self.width {
            if self.height > 0 {
                self.cells[0][x] = Cell::new(TerrainType::Obstacle);
                self.cells[self.height - 1][x] = Cell::new(TerrainType::Obstacle);
            }
        }
        for y in 0..self.height {
            if self.width > 0 {
                self.cells[y][0] = Cell::new(TerrainType::Obstacle);
                self.cells[y][self.width - 1] = Cell::new(TerrainType::Obstacle);
            }
        }
    }

    pub fn get_cell(&self, x: usize, y: usize) -> Option<&Cell> {
        if x < self.width && y < self.height {
            Some(&self.cells[y][x])
        } else {
            None
        }
    }

    pub fn get_cell_mut(&mut self, x: usize, y: usize) -> Option<&mut Cell> {
        if x < self.width && y < self.height {
            Some(&mut self.cells[y][x])
        } else {
            None
        }
    }

    pub fn is_passable(&self, x: usize, y: usize) -> bool {
        match self.get_cell(x, y) {
            Some(cell) => !matches!(cell.terrain, TerrainType::Obstacle | TerrainType::Rock),
            None => false,
        }
    }

    pub fn mark_explored(&mut self, x: usize, y: usize) {
        if x < self.width && y < self.height {
            self.cells[y][x].explored = true;
        }
    }

    pub fn update_robot_position(&mut self, robot_id: u32, x: usize, y: usize) {
        self.robot_positions.insert(robot_id, (x, y));
    }

    // Nouvelle méthode pour épuiser les ressources
    pub fn deplete_resource(&mut self, x: usize, y: usize) {
        if let Some(cell) = self.get_cell_mut(x, y) {
            match cell.terrain {
                TerrainType::Energy => {
                    cell.energy_level = cell.energy_level.saturating_sub(20);
                    if cell.energy_level == 0 {
                        cell.depleted = true;
                        cell.terrain = TerrainType::Empty; // Transformer en case vide
                    }
                },
                TerrainType::Mineral => {
                    cell.mineral_count = cell.mineral_count.saturating_sub(1);
                    if cell.mineral_count == 0 {
                        cell.depleted = true;
                        cell.terrain = TerrainType::Empty; // Transformer en case vide
                    }
                },
                _ => {}
            }
        }
    }

    pub fn display(&self) {
        println!("Carte EREEA ({}x{}):", self.width, self.height);
        for y in 0..self.height {
            for x in 0..self.width {
                let cell = &self.cells[y][x];
                let symbol = match cell.terrain {
                    TerrainType::Empty => if cell.explored { '·' } else { ' ' },
                    TerrainType::Rock => '#',
                    TerrainType::Mineral => 'M',
                    TerrainType::Energy => 'E',
                    TerrainType::Interest => '!',
                    TerrainType::Obstacle => 'X',
                };
                print!("{}", symbol);
            }
            println!();
        }
    }

    pub fn display_with_robots(&self, robots: &[crate::robot::Robot]) {
        println!("Carte EREEA avec robots:");
        for y in 0..self.height {
            for x in 0..self.width {
                // Vérifier s'il y a un robot à cette position
                let mut robot_here = false;
                for robot in robots.iter() {
                    if robot.x == x && robot.y == y {
                        print!("{}", robot.get_symbol());
                        robot_here = true;
                        break;
                    }
                }

                if !robot_here {
                    let cell = &self.cells[y][x];
                    let symbol = match cell.terrain {
                        TerrainType::Empty => if cell.explored { '·' } else { ' ' },
                        TerrainType::Rock => '#',
                        TerrainType::Mineral => 'M',
                        TerrainType::Energy => 'E',
                        TerrainType::Interest => '!',
                        TerrainType::Obstacle => 'X',
                    };
                    print!("{}", symbol);
                }
            }
            println!();
        }

        // Afficher les statistiques des robots
        println!("\nÉtat des robots:");
        for robot in robots.iter() {
            println!("Robot {} ({}): Position({},{}) Énergie:{} État:{:?}",
                     robot.id, robot.robot_type.name(), robot.x, robot.y,
                     robot.energy, robot.behavior.get_current_state());
        }
    }
}