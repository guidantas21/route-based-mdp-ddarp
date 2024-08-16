use serde::Deserialize;
use std::cmp::Ordering;

#[derive(Debug, Deserialize)]
pub struct RequestInstance {
    pub number_requests: usize,
    pub requests: Vec<Request>,
}

#[derive(Debug, Deserialize, Clone, PartialEq)]
pub struct Request {
    reveal_time: f32,
    pickup_location: usize,
    drop_off_location: usize,
    earliest_tw: f32,
    latest_tw: f32,
}

impl Eq for Request {}

impl Ord for Request {
    fn cmp(&self, other: &Self) -> Ordering {
        other
            .reveal_time
            .partial_cmp(&self.reveal_time)
            .unwrap_or(Ordering::Equal)
    }
}

impl PartialOrd for Request {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Request {
    pub fn new(
        reveal_time: f32,
        pickup_location: usize,
        drop_off_location: usize,
        earliest_tw: f32,
        latest_tw: f32,
    ) -> Self {
        Self {
            pickup_location,
            drop_off_location,
            earliest_tw,
            latest_tw,
            reveal_time,
        }
    }

    // GETTERS /////

    #[inline(always)]
    pub fn get_reveal_time(&self) -> f32 {
        self.reveal_time
    }

    #[inline(always)]
    pub fn get_pickup_location(&self) -> usize {
        self.pickup_location
    }

    #[inline(always)]
    pub fn get_drop_off_location(&self) -> usize {
        self.drop_off_location
    }

    #[inline(always)]
    pub fn get_earliest_tw(&self) -> f32 {
        self.earliest_tw
    }

    #[inline(always)]
    pub fn get_latest_tw(&self) -> f32 {
        self.latest_tw
    }
}
