use axum::{
    extract::Path,
    response::IntoResponse,
    routing::{get, post},
    Json, Router,
};
use serde_json::json;

use crate::model::vehicle::VehicleUpsert;
use crate::service::vehicle_service::*;

pub fn config_endpoints() -> Router {
    async fn map_get_vehicles() -> impl IntoResponse {
        format!("{}", json!(get_vehicles()))
    }

    async fn map_get_vehicle(Path(vehicle_id): Path<i64>) -> impl IntoResponse {
        format!("{}", json!(get_vehicle(vehicle_id)))
    }

    async fn map_add_vehicle(vehicle: Json<VehicleUpsert>) -> impl IntoResponse {
        format!("{}", json!(add_vehicle(vehicle.0)))
    }

    async fn map_update_vehicle(
        Path(vehicle_id): Path<i64>,
        vehicle: Json<VehicleUpsert>,
    ) -> impl IntoResponse {
        format!("{}", json!(update_vehicle(vehicle_id, vehicle.0)))
    }

    async fn map_delete_vehicle(Path(vehicle_id): Path<i64>) -> impl IntoResponse {
        format!("{}", json!(delete_vehicle(vehicle_id)))
    }

    Router::new()
        .route("/vehicles", get(map_get_vehicles))
        .route("/vehicle/:vehicleid", get(map_get_vehicle))
        .route("/vehicle/add", post(map_add_vehicle))
        .route("/vehicle/update/:vehicleid", post(map_update_vehicle))
        .route("/vehicle/delete/:vehicleid", post(map_delete_vehicle))
}

/**
 * Unit test cases
 */
#[cfg(test)]
mod tests {
    use super::*;

    /**
     * Scenario:
     * Executes config_endpoints with valid API server
     * Expectation:
     * A set of endpoints should be created
     */
    #[test]
    fn when_config_endpoints_with_api_server_should_add_endpoints() {
        // TODO to be implemented
        assert_eq!(true, true);
    }
}
