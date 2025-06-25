use rand::SeedableRng;
use rand::rngs::StdRng;

pub struct NoiseGenerator {
    _rng: StdRng,
}

impl NoiseGenerator {
    pub fn new() -> Self {
        NoiseGenerator {
            _rng: StdRng::seed_from_u64(42),
        }
    }

    pub fn perlin_2d(&self, x: f64, y: f64) -> f64 {
        // Générateur de bruit simplifié et sûr
        let xi = (x * 100.0) as i32;
        let yi = (y * 100.0) as i32;

        // Hash simple et sûr
        let hash = self.simple_hash(xi, yi);

        // Convertir en valeur entre -1.0 et 1.0
        ((hash % 1000) as f64 / 500.0) - 1.0
    }

    fn simple_hash(&self, x: i32, y: i32) -> u32 {
        // Utilisation d'une méthode de hash simple sans risque de débordement
        let mut hash = (x as u32).wrapping_mul(73856093) ^ (y as u32).wrapping_mul(19349663);
        hash = hash.wrapping_mul(83492791);
        hash ^= hash >> 16;
        hash = hash.wrapping_mul(85493791);
        hash ^= hash >> 11;
        hash = hash.wrapping_mul(73856093);
        hash ^= hash >> 15;
        hash
    }
}