mod mdp;
mod policies;

use std::error::Error;

use crate::{
    mdp::{
        config::{Config, EnvironmentConfig, InformationModelConfig, NetworkConfig, VehicleConfig},
        environment::Environment,
        info_model::InformationModel,
        network::Network,
        request::Request,
        route_plan::RoutePlan,
        vehicle::Vehicle,
    },
    policies::dummy_policy::dummy_policy,
};

fn main() -> Result<(), Box<dyn Error>> {
    let config = Config::load_from_file("data/instances/test/config.toml")?;

    let mut vehicle = Vehicle::new(config.get_vehicle());
    let network = Network::new(config.get_network(), vehicle.get_average_velocity());
    let mut env = Environment::new(config.get_environment(), &network, &mut vehicle);
    let mut info_model = InformationModel::new(config.get_information_model());

    env.init();

    while !env.is_horizon_reached() {
        let mut revealed_requests: Vec<Request> = info_model.reveal_requests(env.get_time());

        if !revealed_requests.is_empty() {
            println!("{:#?}", revealed_requests);
        }

        env.update(&mut revealed_requests, 0.0);

        // IMPROVE
        let new_route_plan = dummy_policy(env.vehicle.route_plan.clone());

        env.step(new_route_plan);
    }

    Ok(())
}
