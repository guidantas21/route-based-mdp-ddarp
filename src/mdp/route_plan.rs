#[derive(Debug, Clone)]
pub enum LocationType {
    Pickup,
    Dropoff,
    Depot,
}

#[derive(Debug, Clone)]
pub struct RouteNode {
    pub location: usize,
    pub location_type: LocationType,
    pub planned_arrival_time: f32,
    pub pickup_time: f32,
    pub earliest_service_time: f32,
    pub latest_service_time: f32,
}

impl RouteNode {
    pub fn new_depot(depot: usize) -> Self {
        Self {
            location_type: LocationType::Depot,
            location: depot,
            planned_arrival_time: 0.0,
            pickup_time: 0.0,
            earliest_service_time: f32::NEG_INFINITY,
            latest_service_time: f32::INFINITY,
        }
    }

    pub fn new_pickup(
        location: usize,
        planned_arrival_time: f32,
        pickup_time: f32,
        earliest_service_time: f32,
    ) -> Self {
        Self {
            location_type: LocationType::Dropoff,
            location,
            planned_arrival_time,
            pickup_time,
            earliest_service_time,
            latest_service_time: f32::INFINITY,
        }
    }

    pub fn new_dropoff(
        location: usize,
        planned_arrival_time: f32,
        pickup_time: f32,
        latest_service_time: f32,
    ) -> Self {
        Self {
            location_type: LocationType::Dropoff,
            location,
            planned_arrival_time,
            pickup_time,
            latest_service_time,
            earliest_service_time: f32::NEG_INFINITY,
        }
    }
}

#[derive(Debug, Clone)]
pub struct RoutePlan {
    pub route: Vec<RouteNode>,
    pub reward: f32,
}

impl RoutePlan {
    #[inline(always)]
    pub fn new() -> Self {
        Self {
            route: Vec::new(),
            reward: 0.0,
        }
    }

    // METHODS /////

    pub fn reward_time_constraints(
        &mut self,
        current_node_index: usize,
        max_service_time: f32,
    ) -> f32 {
        let mut total_reward: f32 = 0.0;

        for i in current_node_index..self.route.len() {
            let node = &self.route[i];

            total_reward += (node.earliest_service_time - node.planned_arrival_time).max(0.0)
                + (node.planned_arrival_time - node.latest_service_time).max(0.0)
                + (node.planned_arrival_time - node.pickup_time - max_service_time).max(0.0);
        }
        total_reward
    }
}
