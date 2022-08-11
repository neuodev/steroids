use log::info;

use crate::Database;
use tokio_postgres::{Error};
use std::{process};
use tokio::fs;

use log::{error};

pub async fn migrate(client: Database) -> Result<(), Error> {
    info!("Start migrating...");
    let query = fs::read_to_string("./sql/init.sql").await.unwrap_or_else(|e| {
        error!("Failed to read schema file: {}", e);
        process::exit(1);
    });
    info!("Schema file loaded successfully");
    client.lock().unwrap().batch_execute(query.as_str()).await
}