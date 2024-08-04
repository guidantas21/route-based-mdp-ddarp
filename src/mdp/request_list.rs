use serde::{Deserialize, Serialize};
use std::{
    fs::File,
    io::{BufReader, Result},
};

use crate::mdp::request::Request;

#[derive(Deserialize, Serialize, Debug)]
pub struct RequestList {
    number_requests: usize,
    requests: Vec<Request>,
}

impl RequestList {
    pub fn new(requests: Vec<Request>) -> Self {
        Self {
            number_requests: requests.len(),
            requests,
        }
    }

    pub fn load_from_json(json_file_path: String) -> Result<Self> {
        let json_file = File::open(json_file_path)?;
        let reader = BufReader::new(json_file);
        let request_list = serde_json::from_reader(reader)?;

        Ok(request_list)
    }

    pub fn get_requests_at(&mut self, time: f32) -> Vec<Request> {
        let mut revealed_requests: Vec<Request> = Vec::new();

        while self.requests.last().expect("REASON").get_reveal_time() <= time {
            revealed_requests.push(self.requests.pop().expect("REASON"));
        }
        revealed_requests
    }
}
