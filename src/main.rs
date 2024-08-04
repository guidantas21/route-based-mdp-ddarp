mod mdp;

use std::error::Error;

use crate::mdp::{
    config::{Config, EnvironmentConfig, InformationModelConfig, NetworkConfig, VehicleConfig},
    environment::Environment,
    info_model::InformationModel,
    network::Network,
    request::Request,
    request_list::RequestList,
    vehicle::Vehicle,
};

fn main() -> Result<(), Box<dyn Error>> {
    let config = Config::load_from_file("data/instances/test/config.toml")?;

    let vehicle = Vehicle::new(config.get_vehicle());

    let network = Network::new(config.get_network(), vehicle.get_average_velocity());

    let env = Environment::new(config.get_environment(), &network, &vehicle);
    println!("{:#?}", env);

    let info_model = InformationModel::new(config.get_information_model());
    println!("{:#?}", info_model);

    Ok(())
}
