#[derive(Debug)]
pub struct Config
{
    time_horizon_max: f32,
    vehicle_velocity: f32,
    num_locations: usize,
    distance_matrix: Vec<Vec<f32>>,
    max_service_time: f32
}

impl Config {
    pub fn new(
        time_horizon_max: f32,
        max_service_time: f32,
        vehicle_velocity: f32,
        distance_matrix: Vec<Vec<f32>>
    ) -> Self 
    {
      	let num_locations = distance_matrix.len();
		
		Self {
            time_horizon_max,
            max_service_time,
            vehicle_velocity,
            num_locations,
            distance_matrix,
        } 
    }
    
    // GETTERS /////

    #[inline(always)]
    pub fn get_vehicle_velocity(&self) -> f32
    {
        self.vehicle_velocity
    }

    #[inline(always)]
    pub fn get_num_locations(&self) -> usize
    {
        self.num_locations
    }

    #[inline(always)]
    pub fn get_time_horizon_max(&self) -> f32
    {
        self.time_horizon_max
    }
    
    // METHODS /////

    #[inline(always)]
    pub fn distance(&self, i: usize, j:usize) -> f32
    {
        self.distance_matrix[i][j]
    }
}
