use super::TerrainType;

impl TerrainType {
    pub fn is_resource(&self) -> bool {
        matches!(self, TerrainType::Mineral | TerrainType::Energy)
    }

    pub fn is_obstacle(&self) -> bool {
        matches!(self, TerrainType::Rock | TerrainType::Obstacle)
    }

    pub fn exploration_value(&self) -> u32 {
        match self {
            TerrainType::Interest => 100,
            TerrainType::Mineral => 50,
            TerrainType::Energy => 30,
            TerrainType::Empty => 1,
            _ => 0,
        }
    }
}