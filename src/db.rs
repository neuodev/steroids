use log::info;

use crate::Database;
use tokio_postgres::{Error, Client};
use std::{process, sync::MutexGuard};
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

pub async fn register_user(client: &mut MutexGuard<'_, Client>, username: String, password: String) -> Result<u64, Error> {
    // Todo: Password should be incepted
    client.execute(r#"
        INSERT INTO users (username, password)
        values ($1, $2)
    "#, &[
        &username,
        &password,
    ]).await
}