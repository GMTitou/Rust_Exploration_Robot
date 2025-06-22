// src/map/terrain.rs - Utilitaires pour terrains

// Fonctions utilitaires pour les terrains
pub fn terrain_symbol(terrain: &final_project::TerrainType) -> char {
    match terrain {
        final_project::TerrainType::Plaine => '.',
        final_project::TerrainType::Montagne => '^',
        final_project::TerrainType::Cratere => 'O',
        final_project::TerrainType::Obstacle => '#',
    }
}

pub fn movement_cost(terrain: &final_project::TerrainType) -> u32 {
    match terrain {
        final_project::TerrainType::Plaine => 1,
        final_project::TerrainType::Cratere => 2,
        final_project::TerrainType::Montagne => 3,
        final_project::TerrainType::Obstacle => u32::MAX,
    }
}