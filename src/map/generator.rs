use super::{Map, TerrainType, Cell};
use crate::utils::noise::NoiseGenerator;

pub struct MapGenerator {
    noise: NoiseGenerator,
}

impl MapGenerator {
    pub fn new() -> Self {
        MapGenerator {
            noise: NoiseGenerator::new(),
        }
    }

    pub fn generate_alien_terrain(&self, map: &mut Map) {
        // Génération procédurale avec du bruit de Perlin
        for y in 0..map.height {
            for x in 0..map.width {
                let noise_val = self.noise.perlin_2d(x as f64 * 0.05, y as f64 * 0.05);

                map.cells[y][x] = match noise_val {
                    n if n > 0.4 => Cell::new(TerrainType::Rock),
                    n if n > 0.2 => Cell::new(TerrainType::Mineral),
                    n if n < -0.3 => Cell::new(TerrainType::Energy),
                    n if n < -0.5 => Cell::new(TerrainType::Interest),
                    _ => Cell::new(TerrainType::Empty),
                };
            }
        }
    }
}