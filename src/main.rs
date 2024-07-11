mod config;
mod env;

use crate::config::Config;
use crate::env::Environment;

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
    
    println!("{:#?}", config);
}
