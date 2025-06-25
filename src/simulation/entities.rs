use crate::simulation::robot_ai::robot::Robot;
use noise::Perlin;
use serde::ser::SerializeStruct;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::collections::HashMap;

pub const STATION_RECHARGE_RATE: u32 = 50;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ResourceType {
    Energy,
    Mineral,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LocationInfo {
    pub position: (usize, usize),
    pub terrain_type: u8,
    pub resource: Option<(ResourceType, u32)>,
    pub discovered_by: usize,
    pub discovery_time: u64,
    pub confidence: f32,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct InformationConflict {
    pub position: (usize, usize),
    pub current_info: LocationInfo,
    pub new_info: LocationInfo,
    pub conflict_type: ConflictType,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ConflictType {
    ResourceAmountDifference,
    ResourceTypeConflict,
    TerrainMismatch,
    ConfidenceConflict,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ConflictResolution {
    KeepCurrent,
    AcceptNew,
    Merge,
    RequiresManualReview,
}

pub struct Map {
    pub width: usize,
    pub height: usize,
    pub terrain: Vec<Vec<u8>>,
    pub resources: HashMap<(usize, usize), (ResourceType, u32)>,
    pub discovered: Vec<Vec<bool>>,
    pub noise: Perlin,
    pub seed: u64,
}

impl Serialize for Map {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("Map", 6)?;
        state.serialize_field("width", &self.width)?;
        state.serialize_field("height", &self.height)?;
        state.serialize_field("terrain", &self.terrain)?;

        let resources_vec: Vec<((usize, usize), (ResourceType, u32))> = self
            .resources
            .iter()
            .map(|(&k, v)| (k, v.clone()))
            .collect();
        state.serialize_field("resources", &resources_vec)?;

        state.serialize_field("discovered", &self.discovered)?;
        state.serialize_field("seed", &self.seed)?;
        state.end()
    }
}

impl<'de> Deserialize<'de> for Map {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        struct MapHelper {
            width: usize,
            height: usize,
            terrain: Vec<Vec<u8>>,
            resources: Vec<((usize, usize), (ResourceType, u32))>,
            discovered: Vec<Vec<bool>>,
            seed: u64,
        }

        let helper = MapHelper::deserialize(deserializer)?;
        let resources: HashMap<(usize, usize), (ResourceType, u32)> =
            helper.resources.into_iter().collect();
        let noise = Perlin::new(helper.seed as u32);

        Ok(Map {
            width: helper.width,
            height: helper.height,
            terrain: helper.terrain,
            resources,
            discovered: helper.discovered,
            noise,
            seed: helper.seed,
        })
    }
}

pub struct Station {
    pub resources: HashMap<ResourceType, u32>,
    pub discoveries: u32,
    pub x: usize,
    pub y: usize,
}

impl Station {
    pub fn new(x: usize, y: usize) -> Self {
        Self {
            resources: HashMap::new(),
            discoveries: 0,
            x,
            y,
        }
    }

    pub fn position(&self) -> (usize, usize) {
        (self.x, self.y)
    }

    pub fn receive_resource(&mut self, resource_type: ResourceType, amount: u32) {
        let resource_type_clone = resource_type.clone();
        *self.resources.entry(resource_type_clone).or_insert(0) += amount;
    }

    pub fn get_resource_amount(&self, resource_type: &ResourceType) -> u32 {
        *self.resources.get(resource_type).unwrap_or(&0)
    }

    pub fn robot_at_station(&self, robot_position: (usize, usize)) -> bool {
        robot_position == self.position()
    }

    pub fn recharge_robot(&mut self, robot: &mut Robot) -> Result<u32, &'static str> {
        let energy_available = self.get_resource_amount(&ResourceType::Energy);
        if energy_available == 0 {
            return Err("No energy available for recharging");
        }

        let energy_needed = robot.max_energy() - robot.energy();
        if energy_needed == 0 {
            return Err("Robot is already at full energy");
        }

        let energy_to_transfer = std::cmp::min(energy_needed, STATION_RECHARGE_RATE);
        let actual_transfer = std::cmp::min(energy_to_transfer, energy_available);

        robot.recharge(actual_transfer);
        self.resources
            .insert(ResourceType::Energy, energy_available - actual_transfer);

        Ok(actual_transfer)
    }

    pub fn can_recharge(&self) -> bool {
        self.get_resource_amount(&ResourceType::Energy) > 0
    }
}