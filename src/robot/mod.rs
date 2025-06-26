pub mod ai;
pub mod communication;
pub mod core;
pub mod movement;

// Re-exports principaux pour faciliter l'utilisation
pub use core::{Robot, RobotType, RobotState, Direction};
pub use ai::{RobotBehavior, create_behavior, Task, TaskType, ExploreTask, HarvestTask, AnalyzeTask, SearchUtils};
pub use movement::{Executor, Pathfinder};