// src/lib.rs - Types de base pour EREEA
use std::collections::HashMap;

// Types de terrain sur la planète
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TerrainType {
    Plaine,
    Montagne,
    Cratere,
    Obstacle,
}

// Types de ressources à collecter
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ResourceType {
    Energie,
    Mineraux,
    LieuxInteret, // Sites scientifiques
}

// Position sur la carte
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Position {
    pub x: usize,
    pub y: usize,
}

impl Position {
    pub fn new(x: usize, y: usize) -> Self {
        Position { x, y }
    }

    pub fn distance_to(&self, other: &Position) -> f64 {
        let dx = (self.x as f64 - other.x as f64).abs();
        let dy = (self.y as f64 - other.y as f64).abs();
        (dx * dx + dy * dy).sqrt()
    }
}

// Cellule de la carte
#[derive(Debug, Clone)]
pub struct Cell {
    pub terrain: TerrainType,
    pub resources: HashMap<ResourceType, u32>,
    pub explored: bool,
    pub occupied_by: Option<usize>, // ID du robot présent
}

impl Cell {
    pub fn new(terrain: TerrainType) -> Self {
        Cell {
            terrain,
            resources: HashMap::new(),
            explored: false,
            occupied_by: None,
        }
    }

    pub fn add_resource(&mut self, resource_type: ResourceType, amount: u32) {
        *self.resources.entry(resource_type).or_insert(0) += amount;
    }

    pub fn is_passable(&self) -> bool {
        self.terrain != TerrainType::Obstacle
    }
}

// Modules spécialisés des robots
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum RobotModule {
    AnalyseChimique,
    ImageHauteResolution,
    Deplacement,
    Communication,
    CollecteEnergie,
    CollecteMineraux,
    CollecteDonnees,
}

// Comportements des robots
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum RobotBehavior {
    Explorateur,  // Explore les zones inconnues
    Collecteur,   // Collecte les ressources
    Scientifique, // Analyse les lieux d'intérêt
}