use axum::{
    extract::Path,
    response::IntoResponse,
    routing::{get, post},
    Json, Router,
};
use serde_json::json;

use crate::model::user::UserUpsert;
use crate::service::user_service::*;

pub fn config_endpoints() -> Router {
    async fn map_get_users() -> impl IntoResponse {
        format!("{}", json!(get_users()))
    }

    async fn map_get_user(Path(user_id): Path<i64>) -> impl IntoResponse {
        format!("{}", json!(get_user(user_id)))
    }

    async fn map_add_user(user: Json<UserUpsert>) -> impl IntoResponse {
        format!("{}", json!(add_user(user.0)))
    }

    async fn map_update_user(
        Path(user_id): Path<i64>,
        user: Json<UserUpsert>,
    ) -> impl IntoResponse {
        format!("{}", json!(update_user(user_id, user.0)))
    }

    async fn map_delete_user(Path(user_id): Path<i64>) -> impl IntoResponse {
        format!("{}", json!(delete_user(user_id)))
    }

    Router::new()
        .route("/users", get(map_get_users))
        .route("/user/:userid", get(map_get_user))
        .route("/user/add", post(map_add_user))
        .route("/user/update/:userid", post(map_update_user))
        .route("/user/delete/:userid", post(map_delete_user))
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
