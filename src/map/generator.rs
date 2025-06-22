// src/map/generator.rs - Générateur de cartes simple
use crate::{Cell, TerrainType, ResourceType};

pub struct MapGenerator {
    seed: u64,
    width: usize,
    height: usize,
}

impl MapGenerator {
    pub fn new(seed: u64, width: usize, height: usize) -> Self {
        MapGenerator { seed, width, height }
    }

    pub fn generate(&self) -> Vec<Vec<Cell>> {
        let mut map = Vec::with_capacity(self.height);

        for y in 0..self.height {
            let mut row = Vec::with_capacity(self.width);
            for x in 0..self.width {
                let terrain = self.generate_terrain(x, y);
                let mut cell = Cell::new(terrain);

                // Ajouter des ressources basé sur la position
                self.add_resources_by_position(x, y, &mut cell);

                row.push(cell);
            }
            map.push(row);
        }

        map
    }

    fn generate_terrain(&self, x: usize, y: usize) -> TerrainType {
        // Génération simple basée sur la position
        let hash = self.simple_hash(x, y);
        let value = (hash % 100) as f32 / 100.0;

        match value {
            v if v < 0.1 => TerrainType::Obstacle,
            v if v < 0.3 => TerrainType::Montagne,
            v if v < 0.4 => TerrainType::Cratere,
            _ => TerrainType::Plaine,
        }
    }

    fn add_resources_by_position(&self, x: usize, y: usize, cell: &mut Cell) {
        let hash = self.simple_hash(x + 1000, y + 1000);

        match cell.terrain {
            TerrainType::Plaine => {
                if hash % 10 < 3 { // 30% de chance
                    cell.add_resource(ResourceType::Energie, 10 + (hash % 40));
                }
            },
            TerrainType::Montagne => {
                if hash % 10 < 5 { // 50% de chance
                    cell.add_resource(ResourceType::Mineraux, 20 + (hash % 60));
                }
            },
            TerrainType::Cratere => {
                if hash % 10 < 7 { // 70% de chance
                    cell.add_resource(ResourceType::LieuxInteret, 1);
                }
            },
            TerrainType::Obstacle => {
                // Pas de ressources
            }
        }
    }

    fn simple_hash(&self, x: usize, y: usize) -> u32 {
        let mut hash = (x as u32).wrapping_mul(374761393)
            .wrapping_add((y as u32).wrapping_mul(668265263))
            .wrapping_add(self.seed as u32);
        hash ^= hash >> 16;
        hash = hash.wrapping_mul(0x85ebca6b);
        hash ^= hash >> 13;
        hash = hash.wrapping_mul(0xc2b2ae35);
        hash ^= hash >> 16;
        hash
    }
}