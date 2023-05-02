use log::info;

use crate::model::vehicle::{Vehicle, VehicleUpsert};
use crate::repository::*;

pub fn get_vehicles() -> Vec<Vehicle> {
    info!("vehicle_service - get_vehicles - executed");
    vehicle_repository::get_vehicles()
}

pub fn get_vehicle(vehicle_id: i64) -> Vehicle {
    info!(
        "vehicle_service - get_vehicle - executed for vehicle id: {}",
        vehicle_id
    );
    vehicle_repository::get_vehicle(vehicle_id)
}

pub fn add_vehicle(vehicle: VehicleUpsert) -> Vehicle {
    info!(
        "vehicle_service - add_vehicle - executed for vehicle: {:?}",
        vehicle
    );
    vehicle_repository::add_vehicle(vehicle)
}

pub fn update_vehicle(vehicle_id: i64, vehicle: VehicleUpsert) -> Vehicle {
    info!(
        "vehicle_service - update_vehicle - executed for vehicle id: {} - vehicle: {:?}",
        vehicle_id, vehicle
    );
    vehicle_repository::update_vehicle(vehicle_id, vehicle)
}

pub fn delete_vehicle(vehicle_id: i64) -> String {
    info!(
        "vehicle_service - delete_vehicle - executed for vehicle id: {}",
        vehicle_id
    );
    vehicle_repository::delete_vehicle(vehicle_id)
}

/**
 * Unit test cases
 */
#[cfg(test)]
mod tests {
    use super::*;

    /**
     * Scenario:
     * Executes get_vehicles with valid parameters
     * Expectation:
     * A list of vehicles should be retrieved
     */
    #[test]
    fn when_get_vehicles_with_valid_parameters_should_return_vehicle_list() {
        // TODO to be implemented
        assert_eq!(true, true);
    }

    /**
     * Scenario:
     * Executes get_vehicle with valid parameters
     * Expectation:
     * An vehicle should be retrieved
     */
    #[test]
    fn when_get_vehicle_with_valid_parameters_should_return_vehicle() {
        // TODO to be implemented
        assert_eq!(true, true);
    }

    /**
     * Scenario:
     * Executes add_vehicle with valid parameters
     * Expectation:
     * An vehicle should be created
     */
    #[test]
    fn when_add_vehicle_with_valid_parameters_should_create_vehicle() {
        // TODO to be implemented
        assert_eq!(true, true);
    }

    /**
     * Scenario:
     * Executes update_vehicle with valid parameters
     * Expectation:
     * An vehicle should be updated
     */
    #[test]
    fn when_update_vehicle_with_valid_parameters_should_update_vehicle() {
        // TODO to be implemented
        assert_eq!(true, true);
    }

    /**
     * Scenario:
     * Executes delete_vehicle with valid parameters
     * Expectation:
     * An vehicle should be deleted
     */
    #[test]
    fn when_delete_vehicle_with_valid_parameters_should_delete_vehicle() {
        // TODO to be implemented
        assert_eq!(true, true);
    }
}
