use crate::mdp::{config::VehicleConfig, request::Request, route_plan::RoutePlan};

#[derive(Debug)]
pub struct InProcessRequest {
    request: Request,
    pick_time: f32,
}

#[derive(Debug)]
pub struct Vehicle<'a> {
    config: &'a VehicleConfig,
    pub route_plan: RoutePlan,
    pub arrival_time: f32,
    pub location: usize,
    in_process_requests: Vec<InProcessRequest>,
}

impl<'a> Vehicle<'a> {
    pub fn new(config: &'a VehicleConfig) -> Self {
        Self {
            config,
            route_plan: RoutePlan::new(),
            arrival_time: 0.0,
            location: 0,
            in_process_requests: Vec::new(),
        }
    }

    // GETTERS /////
    #[inline(always)]
    pub fn get_average_velocity(&self) -> f32 {
        self.config.get_average_velocity()
    }

    // METHODS /////

    pub fn update(&mut self, arrival_time: f32) {
        self.arrival_time = arrival_time;
        self.location = self.route_plan.get_next_location();
    }
}
