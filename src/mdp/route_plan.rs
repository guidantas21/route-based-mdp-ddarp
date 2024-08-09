use std::cmp::max;

#[derive(Debug, Clone)]
pub struct RouteNode {
    pub planned_arrival_time: f32,
    pub pickup_time: f32,
    pub earliest_service_time: f32,
    pub latest_service_time: f32,
}

#[derive(Debug, Clone)]
pub struct RoutePlan {
    route: Vec<RouteNode>,
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