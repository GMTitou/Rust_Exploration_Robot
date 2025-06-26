use crate::maps::entities::ResourceType;

#[derive(Debug, Clone, PartialEq)]
pub struct Task {
    pub task_type: TaskType,
    pub target_position: Option<(usize, usize)>,
    pub priority: u8,
}

#[derive(Debug, Clone, PartialEq)]
pub enum TaskType {
    Explore(ExploreTask),
    Harvest(HarvestTask),
    Analyze(AnalyzeTask),
    ReturnToStation,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ExploreTask {
    pub target_area: (usize, usize),
    pub radius: usize,
}

#[derive(Debug, Clone, PartialEq)]
pub struct HarvestTask {
    pub resource_type: ResourceType,
    pub target_position: (usize, usize),
}

#[derive(Debug, Clone, PartialEq)]
pub struct AnalyzeTask {
    pub target_position: (usize, usize),
    pub analysis_type: AnalysisType,
}

#[derive(Debug, Clone, PartialEq)]
pub enum AnalysisType {
    Chemical,
}