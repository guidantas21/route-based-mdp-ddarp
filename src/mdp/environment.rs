use crate::mdp::{config::EnvironmentConfig, network::Network, vehicle::Vehicle};

#[derive(Debug)]
pub struct Environment<'a> {
    config: &'a EnvironmentConfig,
    network: &'a Network<'a>,
    vehicle: &'a Vehicle<'a>,
    time: f32,
    objective: f32,
    decision_point: usize,
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
}
