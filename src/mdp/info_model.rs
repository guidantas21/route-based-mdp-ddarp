use crate::mdp::{
    config::InformationModelConfig,
    request::{Request, RequestInstance},
    utils::read_matrix_from_file,
};
use serde::Deserialize;
use std::collections::BinaryHeap;

#[derive(Debug)]
pub struct InformationModel<'a> {
    config: &'a InformationModelConfig,
    travel_time_matrix: Vec<Vec<f32>>,
    requests: BinaryHeap<Request>,
}

impl<'a> InformationModel<'a> {
    pub fn new(config: &'a InformationModelConfig) -> Self {
        let travel_time_matrix = read_matrix_from_file(config.get_travel_time_matrix_file())
            .expect("Failed to parse travel time matrix");

        let requests_json = std::fs::read_to_string(config.get_requests_file())
            .expect("Failed to read requests file");

        let request_instance: RequestInstance =
            serde_json::from_str(&requests_json).expect("Failed to parse requests (JSON)");

        let mut requests = BinaryHeap::new();

        for request in request_instance.requests {
            requests.push(request);
        }

        Self {
            config,
            travel_time_matrix,
            requests,
        }
    }

    // REVEAL STOCHASTIC INFORMATION /////

    #[inline(always)]
    pub fn reveal_requests(&mut self, time: f32) -> Vec<Request> {
        let mut revealed_requests: Vec<Request> = Vec::new();

        while let Some(request) = self.requests.peek() {
            if request.get_reveal_time() <= time {
                revealed_requests.push(self.requests.pop().unwrap());
            } else {
                break;
            }
        }
        revealed_requests
    }

    #[inline(always)]
    pub fn reveal_travel_time(&self, i: usize, j: usize) -> f32 {
        self.travel_time_matrix[i][j]
    }
}
