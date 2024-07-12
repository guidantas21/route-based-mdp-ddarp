use crate::request::Request;
use std::cmp::Ordering;

#[derive(Debug)]
pub fn struct InformationModel<'a>
{
    config: &'a Config,
    travel_time_matrix: Vec<Vec<f32>>,
    requests: Vec<Request> // TODO: implement priority queue
}

impl<'a> InformationModel<'a>
{
    pub fn new(
        config: &'a Config, 
        travel_time_matrix: Vec<Vec<f32>>, 
        requests: Vec<Request>) -> Self
    {

        // sort by reveal time in descending order
        requests.sort_by(|a, b| b
                .get_reveal_time()
                .partial_cmp(&a.get_reveal_time())
                .unwrap_or(Ordering::Equal));

        Self {
            config,
            travel_time_matrix,
            requests,
        }
    }

    // REVEAL STOCHASTIC INFORMATION /////
    
    pub fn reveal_requests(&mut self, time: f32) -> Vec<Request>
    {
        let mut revealed_requests = Vec::new();
        
        while self.requests.last().get_reveal_time() <= time
        {
            revealed_requests.push(self.requests.pop());
        }
        revealed_requests
    }
    
    #[inline(always)]
    pub fn reveal_travel_time(&self, i: usize, j: usize) -> f32
    {
        self.travel_time_matrix[i][j]
    }
}
