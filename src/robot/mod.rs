// src/robot/mod.rs - Module de gestion des robots
pub mod robot;
pub mod behavior;
pub mod modules;

// Réexporter les types publics
pub use robot::*;
pub use behavior::*;
pub use modules::*;