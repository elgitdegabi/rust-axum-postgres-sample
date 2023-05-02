use chrono::NaiveDateTime;
use diesel::{AsChangeset, Insertable, Queryable};
use schema::vehicle;
use serde::{Deserialize, Serialize};

use crate::schema;

#[derive(Queryable, Serialize, Deserialize, Debug)]
#[diesel(table_name = vehicle)]
pub struct Vehicle {
    pub vehicle_id: i64,
    pub vin: String,
    pub license_plate: String,
    pub status: String,
    pub vehicle_type: String,
    pub vehicle_date: NaiveDateTime,
    pub vehicle_update: NaiveDateTime,
}

#[derive(Insertable, AsChangeset, Serialize, Deserialize, Debug)]
#[diesel(table_name = vehicle)]
pub struct VehicleUpsert {
    pub vin: Option<String>,
    pub license_plate: Option<String>,
    pub status: Option<String>,
    pub vehicle_type: Option<String>,
    pub vehicle_date: Option<NaiveDateTime>,
    pub vehicle_update: Option<NaiveDateTime>,
}

/**
 * Unit test cases
 */
#[cfg(test)]
mod tests {
    use super::*;

    /**
     * Scenario:
     * Executes User struct validation with valid values
     * Expectation:
     * The struct with proper values should be created
     **/
    #[test]
    fn when_create_vehicle_struct_should_retrieve_struct_values() {
        // TODO to implement
        assert_eq!(1, 1);
    }

    /**
     * Scenario:
     * Executes VehicleUpsert struct validation with populated values
     * Expectation:
     * The struct with proper values should be created
     */
    #[test]
    fn when_create_vehicle_upsert_struct_should_retrieve_struct_values() {
        // TODO to implement
        assert_eq!(1, 1);
    }
}
