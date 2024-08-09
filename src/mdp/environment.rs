use crate::mdp::{config::EnvironmentConfig, network::Network, vehicle::Vehicle, request::Request};

#[derive(Debug)]
pub struct Environment<'a> {
    config: &'a EnvironmentConfig,
    network: &'a Network<'a>,
    vehicle: &'a Vehicle<'a>,
    time: f32,
    objective: f32,
    decision_point: usize,
    outstanding_requests: Vec<Request>,
}

impl<'a> Environment<'a> {
    pub fn new(
        config: &'a EnvironmentConfig,
        network: &'a Network<'a>,
        vehicle: &'a Vehicle<'a>,
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

    pub fn update(&self, route: Vec<Request>) {
        
    }

    pub fn step() {}
}
