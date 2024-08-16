use crate::mdp::{config::VehicleConfig, request::Request, route_plan::RoutePlan};

#[derive(Debug)]
pub struct InProcessRequest {
    request: Request,
    pickup_time: f32,
}

#[derive(Debug)]
pub struct Vehicle<'a> {
    config: &'a VehicleConfig,
    pub route_plan: RoutePlan,
    pub arrival_time: f32,
    pub location: usize,
    num_visited_locations: usize,
    in_process_requests: Vec<InProcessRequest>,
}

impl<'a> Vehicle<'a> {
    pub fn new(config: &'a VehicleConfig) -> Self {
        Self {
            config,
            route_plan: RoutePlan::new(),
            arrival_time: 0.0,
            location: 0,
            num_visited_locations: 0,
            in_process_requests: Vec::new(),
        }
    }

    // GETTERS /////

    #[inline(always)]
    pub fn get_average_velocity(&self) -> f32 {
        self.config.get_average_velocity()
    }

    #[inline(always)]
    pub fn get_next_location(&self) -> usize {
        let next_node_index = self.num_visited_locations + 1;

        if next_node_index >= self.route_plan.route.len() {
            return self.route_plan.route[self.num_visited_locations].location;
        }
        self.route_plan.route[next_node_index].location
    }

    #[inline(always)]
    pub fn get_num_visited_locations(&self) -> usize {
        self.num_visited_locations
    }

    // METHODS /////

    pub fn update(&mut self, arrival_time: f32) {
        self.arrival_time = arrival_time;

        let next_location = self.get_next_location();

        if next_location != self.location {
            self.num_visited_locations += 1;
        }
    }
}
