// src/utils/noise.rs - Utilitaires pour génération procédurale
use noise::{NoiseFn, Perlin, Seedable};
use crate::TerrainType;

pub struct NoiseGenerator {
    perlin: Perlin,
    scale: f64,
}

impl NoiseGenerator {
    pub fn new(seed: u32, scale: f64) -> Self {
        let perlin = Perlin::new(seed);
        NoiseGenerator { perlin, scale }
    }

    /// Génère une valeur de bruit entre 0.0 et 1.0
    pub fn get_noise(&self, x: f64, y: f64) -> f64 {
        let noise_value = self.perlin.get([x * self.scale, y * self.scale]);
        // Normaliser de [-1, 1] vers [0, 1]
        (noise_value + 1.0) / 2.0
    }

    /// Génère un type de terrain basé sur le bruit
    pub fn get_terrain(&self, x: f64, y: f64) -> TerrainType {
        let noise = self.get_noise(x, y);

        match noise {
            n if n < 0.15 => TerrainType::Obstacle,
            n if n < 0.35 => TerrainType::Montagne,
            n if n < 0.45 => TerrainType::Cratere,
            _ => TerrainType::Plaine,
        }
    }

    /// Génère une hauteur pour la carte d'élévation
    pub fn get_elevation(&self, x: f64, y: f64) -> f32 {
        self.get_noise(x, y) as f32 * 100.0 // Élévation entre 0 et 100
    }

    /// Génère une densité de ressources
    pub fn get_resource_density(&self, x: f64, y: f64) -> f32 {
        let noise = self.get_noise(x * 2.0, y * 2.0); // Échelle différente
        noise as f32
    }
}

/// Générateur de bruit multicouche pour des terrains plus complexes
pub struct LayeredNoise {
    generators: Vec<(NoiseGenerator, f64)>, // (générateur, poids)
}

impl LayeredNoise {
    pub fn new(seed: u32) -> Self {
        let generators = vec![
            (NoiseGenerator::new(seed, 0.02), 1.0),      // Grandes structures
            (NoiseGenerator::new(seed + 1, 0.05), 0.5),  // Structures moyennes
            (NoiseGenerator::new(seed + 2, 0.1), 0.25),  // Détails fins
        ];

        LayeredNoise { generators }
    }

    pub fn get_combined_noise(&self, x: f64, y: f64) -> f64 {
        let mut total_value = 0.0;
        let mut total_weight = 0.0;

        for (generator, weight) in &self.generators {
            total_value += generator.get_noise(x, y) * weight;
            total_weight += weight;
        }

        total_value / total_weight
    }

    pub fn get_complex_terrain(&self, x: f64, y: f64) -> TerrainType {
        let noise = self.get_combined_noise(x, y);

        // Distribution plus réaliste
        match noise {
            n if n < 0.1 => TerrainType::Obstacle,
            n if n < 0.3 => TerrainType::Montagne,
            n if n < 0.4 => TerrainType::Cratere,
            _ => TerrainType::Plaine,
        }
    }
}

/// Utilitaires de génération procédurale
pub struct ProceduralUtils;

impl ProceduralUtils {
    /// Interpole linéairement entre deux valeurs
    pub fn lerp(a: f64, b: f64, t: f64) -> f64 {
        a + (b - a) * t
    }

    /// Lisse une valeur avec une courbe en S
    pub fn smooth_step(t: f64) -> f64 {
        t * t * (3.0 - 2.0 * t)
    }

    /// Génère un nombre aléatoire basé sur une position
    pub fn hash_position(x: i32, y: i32, seed: u32) -> f64 {
        let mut hash = (x as u32).wrapping_mul(374761393)
            .wrapping_add((y as u32).wrapping_mul(668265263))
            .wrapping_add(seed);

        hash = hash.wrapping_mul(2654435761);
        hash ^= hash >> 16;
        hash = hash.wrapping_mul(2654435761);
        hash ^= hash >> 16;

        (hash as f64) / (u32::MAX as f64)
    }

    /// Calcule la distance euclidienne entre deux points
    pub fn distance(x1: f64, y1: f64, x2: f64, y2: f64) -> f64 {
        let dx = x2 - x1;
        let dy = y2 - y1;
        (dx * dx + dy * dy).sqrt()
    }

    /// Génère des points d'intérêt aléatoires
    pub fn generate_points_of_interest(
        width: usize,
        height: usize,
        count: usize,
        seed: u32,
    ) -> Vec<(usize, usize)> {
        let mut points = Vec::new();

        for i in 0..count {
            let x_hash = Self::hash_position(i as i32, 0, seed);
            let y_hash = Self::hash_position(i as i32, 1, seed.wrapping_add(1));

            let x = (x_hash * width as f64) as usize;
            let y = (y_hash * height as f64) as usize;

            if x < width && y < height {
                points.push((x, y));
            }
        }

        points
    }
}