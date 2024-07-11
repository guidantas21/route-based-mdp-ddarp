use crate::config::Config;

#[derive(Debug)]
pub struct Environment<'a>
{
    config: &'a Config,
    time: f32,
    objective: f32,
    decision_point: usize,
    approx_travel_time_matrix: Vec<Vec<f32>>,
    num_locations: usize
}


impl<'a> Environment<'a>
{
    pub fn new(config: &'a Config) -> Self
    {
        let num_locations = config.get_num_locations();
        
        // construct a matrix with approximations of travel times 
        // => travel_time[i][j] = distance[i][j] / vehicle_velocity
        let approx_travel_time_matrix: Vec<Vec<f32>> = (0..num_locations)
            .map(|i| {
                (0..num_locations)
                    .map(|j| config.distance(i,j) / config.get_vehicle_velocity())
                    .collect()
            })
            .collect();

        Self {
            config,
            time: 0.0,
            objective: 0.0,
            decision_point: 0,
            approx_travel_time_matrix,
            num_locations,
        }
    }
    
    // GETTERS /////

    #[inline(always)]
    pub fn get_time(&self) -> f32
    {
        self.time
    }
    
    #[inline(always)]
    pub fn get_objective(&self) -> f32
    {
        self.objective
    }
    
    #[inline(always)]
    pub fn get_decision_point(&self) -> usize
    {
        self.decision_point
    }
    
    #[inline(always)]
    pub fn get_num_locations(&self) -> usize
    {
        self.num_locations
    }
    
    // AUXILIAR /////

    #[inline(always)]
    pub fn approx_travel_time(&self, i: usize, j: usize) -> f32
    {
        self.approx_travel_time_matrix[i][j]
    }

    // SIMULATION /////
}
