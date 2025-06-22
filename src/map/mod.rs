// src/map/mod.rs - Module de gestion des cartes
pub mod generator;
pub mod terrain;

// Réexporter les types publics
pub use generator::*;
pub use terrain::*;