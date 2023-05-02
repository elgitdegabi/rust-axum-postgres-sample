use crate::config::constants::HOST;
use crate::controller::*;

use config::database::*;
use dotenv::dotenv;
use log::info;
use std::env;

mod config;
mod controller;
mod model;
mod repository;
mod schema;
mod service;

#[tokio::main]
async fn main() {
    dotenv().ok();

    let log_config_file =
        env::var("LOG4RS_CONFIG_FILE").unwrap_or("logging_config.yaml".to_string());
    log4rs::init_file(log_config_file, Default::default()).unwrap();

    unsafe {
        info!("DB pool - starting...");
        DB_POOL = Some(create_db_pool().await);
        info!("DB pool - started OK: {:?}", DB_POOL.as_ref().unwrap());
    }

    info!("Axum server - starting...");
    axum::Server::bind(&HOST.parse().unwrap())
        .serve(
            health_controller::config_endpoints()
                .merge(user_controller::config_endpoints())
                .merge(vehicle_controller::config_endpoints())
                .into_make_service(),
        )
        .await
        .unwrap();
}
