use serde::Deserialize;
use std::fs;

#[derive(Debug, Deserialize, Clone)]
pub struct EnvironmentConfig {
    max_service_time: f32,
    planning_horizon: f32,
}

impl EnvironmentConfig {
    #[inline(always)]
    pub fn get_max_service_time(&self) -> f32 {
        self.max_service_time
    }

    #[inline(always)]
    pub fn get_planning_horizon(&self) -> f32 {
        self.planning_horizon
    }
}

#[derive(Debug, Deserialize, Clone)]
pub struct NetworkConfig {
    num_locations: usize,
    distance_matrix_file: String,
    depot: usize,
}

impl NetworkConfig {
    #[inline(always)]
    pub fn get_num_locations(&self) -> usize {
        self.num_locations
    }

    #[inline(always)]
    pub fn get_distance_matrix_file(&self) -> String {
        self.distance_matrix_file.clone()
    }

    #[inline(always)]
    pub fn get_depot(&self) -> usize {
        self.depot
    }
}

#[derive(Debug, Deserialize, Clone)]
pub struct VehicleConfig {
    average_velocity: f32,
}

impl VehicleConfig {
    #[inline(always)]
    pub fn get_average_velocity(&self) -> f32 {
        self.average_velocity
    }
}

#[derive(Debug, Deserialize, Clone)]
pub struct InformationModelConfig {
    travel_time_matrix_file: String,
    requests_file: String,
}

impl InformationModelConfig {
    #[inline(always)]
    pub fn get_travel_time_matrix_file(&self) -> String {
        self.travel_time_matrix_file.clone()
    }

    #[inline(always)]
    pub fn get_requests_file(&self) -> String {
        self.requests_file.clone()
    }
}

#[derive(Debug, Deserialize, Clone)]
pub struct Config {
    environment: EnvironmentConfig,
    network: NetworkConfig,
    vehicle: VehicleConfig,
    information_model: InformationModelConfig,
}

impl Config {
    pub fn new(
        environment: EnvironmentConfig,
        network: NetworkConfig,
        vehicle: VehicleConfig,
        information_model: InformationModelConfig,
    ) -> Self {
        Self {
            environment,
            network,
            vehicle,
            information_model,
        }
    }

    pub fn load_from_file(config_file_path: &str) -> Result<Config, Box<dyn std::error::Error>> {
        let config_str = fs::read_to_string(config_file_path)?;
        let config: Config = toml::from_str(&config_str)?;

        Ok(config)
    }

    #[inline(always)]
    pub fn get_environment(&self) -> &EnvironmentConfig {
        &self.environment
    }

    #[inline(always)]
    pub fn get_network(&self) -> &NetworkConfig {
        &self.network
    }

    #[inline(always)]
    pub fn get_vehicle(&self) -> &VehicleConfig {
        &self.vehicle
    }

    #[inline(always)]
    pub fn get_information_model(&self) -> &InformationModelConfig {
        &self.information_model
    }
}
