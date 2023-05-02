use axum::{response::IntoResponse, routing::get, Router};
use serde_json::json;

use crate::config::constants::*;
use crate::model::health::Health;

pub fn config_endpoints() -> Router {
    async fn map_health() -> impl IntoResponse {
        format!(
            "{}",
            json!(Health {
                status: String::from(SERVER_RUNNING_STATUS)
            })
        )
    }

    Router::new().route("/health", get(map_health))
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
