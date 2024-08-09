use crate::mdp::{
    config::InformationModelConfig, request::Request, request_list::RequestList,
    utils::read_matrix_from_file,
};

#[derive(Debug)]
pub struct InformationModel<'a> {
    config: &'a InformationModelConfig,
    travel_time_matrix: Vec<Vec<f32>>,
    requests: RequestList,
}

impl<'a> InformationModel<'a> {
    pub fn new(config: &'a InformationModelConfig) -> Self {
        let travel_time_matrix = read_matrix_from_file(config.get_travel_time_matrix_file());

        let requests = RequestList::load_from_json(config.get_requests_file());

        Self {
            config,
            travel_time_matrix: travel_time_matrix.expect("REASON"),
            requests: requests.expect("REASON"),
        }
    }

    // REVEAL STOCHASTIC INFORMATION /////
    
    #[inline(always)]
    pub fn reveal_requests(&mut self, time: f32) -> Vec<Request> {
        self.requests.get_requests_at(time)
    }

    #[inline(always)]
    pub fn reveal_travel_time(&self, i: usize, j: usize) -> f32 {
        self.travel_time_matrix[i][j]
    }
}
