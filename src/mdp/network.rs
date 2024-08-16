use crate::mdp::{config::NetworkConfig, utils::read_matrix_from_file};

#[derive(Debug)]
pub struct Network<'a> {
    config: &'a NetworkConfig,
    distance_matrix: Vec<Vec<f32>>,
    approx_travel_time_matrix: Vec<Vec<f32>>,
}

impl<'a> Network<'a> {
    pub fn new(config: &'a NetworkConfig, average_vehicle_velocity: f32) -> Self {
        let distance_matrix =
            read_matrix_from_file(config.get_distance_matrix_file()).expect("REASON");

        let approx_travel_time_matrix = distance_matrix
            .iter()
            .map(|row| {
                row.iter()
                    .map(|&distance| distance / average_vehicle_velocity)
                    .collect()
            })
            .collect();

        Self {
            config,
            distance_matrix,
            approx_travel_time_matrix,
        }
    }

    pub fn get_depot(&self) -> usize {
        self.config.get_depot()
    }

    #[inline(always)]
    pub fn distance(&self, i: usize, j: usize) -> f32 {
        self.distance_matrix[i][j]
    }

    #[inline(always)]
    pub fn approx_travel_time(&self, i: usize, j: usize) -> f32 {
        self.approx_travel_time_matrix[i][j]
    }
}
