use self::schema::vehicle::dsl::*;
use diesel::prelude::*;
use diesel::{QueryDsl, RunQueryDsl};
use log::info;

use crate::config::constants::DELETE_OK_STATUS;
use crate::config::database::get_pool_connection;
use crate::config::pagination::Pagination;
use crate::model::vehicle::{Vehicle, VehicleUpsert};
use crate::schema;

pub fn get_vehicles(pagination: Pagination) -> Vec<Vehicle> {
    let result = vehicle
        .limit(pagination.per_page)
        .offset(pagination.page)
        .get_results::<Vehicle>(&mut get_pool_connection());
    info!("vehicle_repository - get_vehicles - executed");

    result.unwrap()
}

pub fn get_vehicle(vehicle_id_value: i64) -> Vehicle {
    let result = vehicle
        .filter(vehicle_id.eq(vehicle_id_value))
        .get_result::<Vehicle>(&mut get_pool_connection());
    info!(
        "vehicle_repository - get_vehicle - executed for vehicle id: {}",
        vehicle_id_value
    );

    result.unwrap()
}

pub fn add_vehicle(vehicle_to_add: VehicleUpsert) -> Vehicle {
    info!(
        "vehicle_repository - add_vehicle - executed for vehicle: {:?}",
        vehicle_to_add
    );
    diesel::insert_into(vehicle)
        .values(&vehicle_to_add)
        .get_result(&mut get_pool_connection())
        .expect("add_vehicle execution fails")
}

pub fn update_vehicle(vehicle_id_value: i64, vehicle_to_update: VehicleUpsert) -> Vehicle {
    info!(
        "vehicle_repository - update_vehicle - executed for vehicle id: {} - vehicle: {:?}",
        vehicle_id_value, vehicle_to_update
    );
    diesel::update(vehicle)
        .filter(vehicle_id.eq(vehicle_id_value))
        .set(&vehicle_to_update)
        .get_result(&mut get_pool_connection())
        .expect("update_vehicle execution fails")
}

pub fn delete_vehicle(vehicle_id_value: i64) -> String {
    diesel::delete(vehicle)
        .filter(vehicle_id.eq(vehicle_id_value))
        .execute(&mut get_pool_connection())
        .expect("delete_vehicle execution fails");
    info!(
        "vehicle_repository - delete_vehicle - executed for vehicle id: {}",
        vehicle_id_value
    );

    String::from(DELETE_OK_STATUS)
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
     * A list of vehicle should be retrieved
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
