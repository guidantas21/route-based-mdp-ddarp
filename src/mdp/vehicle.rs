use crate::mdp::config::VehicleConfig;

#[derive(Debug)]
pub struct Vehicle<'a> {
    config: &'a VehicleConfig,
    route: Vec<usize>,
    arrival_time: f32,
    location: usize,
}

impl<'a> Vehicle<'a> {
    pub fn new(config: &'a VehicleConfig) -> Self {
        Self {
            config,
            route: Vec::new(),
            arrival_time: 0.0,
            location: 0,
        }
    }

    // GETTERS /////

    #[inline(always)]
    pub fn get_location(&self) -> usize {
        self.location
    }

    #[inline(always)]
    pub fn get_arrival_time(&self) -> f32 {
        self.arrival_time
    }

    #[inline(always)]
    pub fn get_route(&self) -> Vec<usize> {
        self.route.clone()
    }

    pub fn get_average_velocity(&self) -> f32 {
        self.config.get_average_velocity()
    }

    // SETTERS /////

    #[inline(always)]
    pub fn set_route(&mut self, new_route: Vec<usize>) {
        self.route = new_route;
    }
}
