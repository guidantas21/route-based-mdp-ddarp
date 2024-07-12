use crate::config::Config;
use crate::request::Request;

#[derive(Debug)]
pub struct Vehicle<'a>
{
    config: &'a Config,
    route: Vec<usize>,
    arrival_time: f32,
    location: usize,
    requests: Vec<Request>
}

impl<'a> Vehicle<'a>
{
    pub fn new(config: &'a Config) -> Self
    {
        Self {
            config,
            route: Vec::new(),
            arrival_time: 0.0,
            location: 0,
            requests: Vec::new()
        }
    }
    
    // GETTERS /////
    
    #[inline(always)]
    pub fn get_location(&self) -> usize
    {
        self.location
    }

    #[inline(always)]
    pub fn get_arrival_time(&self) -> f32
    {
        self.arrival_time
    }
    
    #[inline(always)]
    pub fn get_route(&self) -> Vec<usize>
    {
        self.route.clone()
    }

    // SETTERS /////
    
    #[inline(always)]
    pub fn set_route(&mut self, new_route: Vec<usize>)
    {
        self.route = new_route;
    }
}
