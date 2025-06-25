use crate::simulation::entities::{Map, ResourceType};

pub struct SearchUtils;

impl SearchUtils {
    pub fn radius_search<F>(
        robot_x: usize,
        robot_y: usize,
        search_radius: i32,
        map: &Map,
        predicate: F,
    ) -> Option<(usize, usize)>
    where
        F: Fn(usize, usize, &Map) -> bool,
    {
        let robot_x = robot_x as i32;
        let robot_y = robot_y as i32;

        for radius in 1..=search_radius {
            for dx in -radius..=radius {
                for dy in -radius..=radius {
                    let x = robot_x + dx;
                    let y = robot_y + dy;

                    if x >= 0 && y >= 0 && (x as usize) < map.width && (y as usize) < map.height {
                        let x = x as usize;
                        let y = y as usize;

                        if predicate(x, y, map) {
                            return Some((x, y));
                        }
                    }
                }
            }
        }
        None
    }

    pub fn find_nearest_resource(
        robot_x: usize,
        robot_y: usize,
        search_radius: i32,
        map: &Map,
        allowed_types: &[ResourceType],
    ) -> Option<((usize, usize), ResourceType)> {
        Self::radius_search(robot_x, robot_y, search_radius, map, |x, y, map| {
            if let Some((resource_type, _amount)) = map.resources.get(&(x, y)) {
                allowed_types.contains(resource_type)
            } else {
                false
            }
        })
        .and_then(|(x, y)| {
            map.resources
                .get(&(x, y))
                .map(|(resource_type, _)| ((x, y), resource_type.clone()))
        })
    }

    pub fn find_nearest_unexplored(
        robot_x: usize,
        robot_y: usize,
        search_radius: i32,
        map: &Map,
    ) -> Option<(usize, usize)> {
        Self::radius_search(robot_x, robot_y, search_radius, map, |x, y, map| {
            !map.discovered[y][x] && map.terrain[y][x] == 0
        })
    }

    pub fn find_nearest_scientific_interest(
        robot_x: usize,
        robot_y: usize,
        search_radius: i32,
        map: &Map,
    ) -> Option<(usize, usize)> {
        Self::radius_search(robot_x, robot_y, search_radius, map, |x, y, map| {
            matches!(
                map.resources.get(&(x, y)),
                Some((ResourceType::Energy, _))
            )
        })
    }
}