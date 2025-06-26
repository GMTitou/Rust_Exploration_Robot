pub mod entities;
pub mod terrain;
pub mod generation;
pub mod visualization;

// Re-exports pour faciliter l'utilisation
pub use entities::{Map, Station, ResourceType, LocationInfo, InformationConflict, ConflictType, ConflictResolution, STATION_RECHARGE_RATE};
pub use terrain::TerrainType;
pub use generation::{MapConstants, MapError, MapResult};
pub use visualization::{MapVisualizer, App};