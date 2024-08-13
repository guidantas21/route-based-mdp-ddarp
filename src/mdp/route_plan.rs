use std::cmp::max;

#[derive(Debug, Clone)]
pub struct RouteNode {
    pub location: usize,
    pub planned_arrival_time: f32,
    pub pickup_time: f32,
    pub earliest_service_time: f32,
    pub latest_service_time: f32,
}

#[derive(Debug, Clone)]
pub struct RoutePlan {
    pub route: Vec<RouteNode>,
    reward: f32,
}

impl RoutePlan {
    #[inline(always)]
    pub fn new() -> Self {
        Self {
            route: Vec::new(),
            reward: 0.0,
        }
    }

    // GETTERS /////

    #[inline(always)]
    pub fn get_reward(&self) -> f32 {
        self.reward
    }

    #[inline(always)]
    pub fn get_next_location(&self) -> usize {
        self.route[0].location
    }

    // METHODS /////

    #[inline(always)]
    pub fn is_empty(&self) -> bool {
        self.route.is_empty()
    }

    pub fn update_reward(&mut self, max_service_time: f32) {
        let mut total_reward: f32 = 0.0;

        for node in &self.route {
            total_reward += (node.earliest_service_time - node.planned_arrival_time).max(0.0)
                + (node.planned_arrival_time - node.latest_service_time).max(0.0)
                + (node.planned_arrival_time - node.pickup_time - max_service_time).max(0.0);
        }
        self.reward = total_reward;
    }
}
