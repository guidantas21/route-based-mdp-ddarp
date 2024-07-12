mod config;
mod env;
mod vehicle;
mod request;
mod info_model;

use crate::config::Config;
use crate::env::Environment;
use crate::info_model::InformationModel;

fn main() 
{
    let time_horizon_max: f32 = 100.0;
    let vehicle_velocity: f32 = 12.0; 
    let max_service_time: f32 = 40.0;
    let distance_matrix: Vec<Vec<f32>> = vec![
        vec![0.0, 1.0, 2.0],
        vec![1.0, 0.0, 1.5],
        vec![2.0, 1.5, 0.0],
    ];

    let config = Config::new(
        time_horizon_max, 
        max_service_time, 
        vehicle_velocity, 
        distance_matrix
    );
    
    println!("{:#?}", config);

    let mut env = Environment::new(&config);
    
    println!("{:#?}", env);
    
    let travel_time_matrix: Vec<Vec<f32>> = vec![
        vec![0.0, 2.0, 3.0],
        vec![2.0, 0.0, 1.8],
        vec![3.0, 1.8, 0.0],
    ];

    let mut info_model
}
