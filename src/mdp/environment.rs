use crate::mdp::{
    config::EnvironmentConfig, network::Network, request::Request, route_plan::RoutePlan,
    vehicle::Vehicle,
};

#[derive(Debug)]
pub struct Environment<'a> {
    config: &'a EnvironmentConfig,
    network: &'a Network<'a>,
    vehicle: &'a mut Vehicle<'a>,
    time: f32,
    objective: f32,
    decision_point: usize,
    outstanding_requests: Vec<Request>,
}

impl<'a> Environment<'a> {
    pub fn new(
        config: &'a EnvironmentConfig,
        network: &'a Network<'a>,
        vehicle: &'a mut Vehicle<'a>,
    ) -> Self {
        Self {
            config,
            network,
            vehicle,
            time: 0.0,
            objective: 0.0,
            decision_point: 0,
            outstanding_requests: Vec::new(),
        }
    }

    // GETTERS /////

    #[inline(always)]
    pub fn get_time(&self) -> f32 {
        self.time
    }

    #[inline(always)]
    pub fn get_objective(&self) -> f32 {
        self.objective
    }

    #[inline(always)]
    pub fn get_decision_point(&self) -> usize {
        self.decision_point
    }

    // MDP METHODS /////

    pub fn init(&mut self) {
        self.decision_point = 0;
        self.time = 0.0;
        self.objective = 0.0;
    }

    pub fn update(&mut self, revealed_requests: &mut Vec<Request>, revealed_travel_time: f32) {
        self.outstanding_requests.append(revealed_requests);
        self.time += revealed_travel_time;
        self.vehicle.update(self.time);
    }

    pub fn step(&mut self, route_plan: RoutePlan) {
        self.vehicle.route_plan = route_plan;

        if self.vehicle.route_plan.is_empty() == false {
            self.objective += self.vehicle.route_plan.get_reward();
        } else {
            self.time += 1.0;
        }
        self.decision_point += 1;
    }
}
