use log::info;

use crate::Database;
use tokio_postgres::{Error};
use std::{fs, process};
use log::{error};

pub async fn migrate(client: Database) -> Result<(), Error> {
    let query = fs::read_to_string("./sql/init.sql").unwrap_or_else(|e| {
        error!("Failed to read schema file: {}", e);
        process::exit(1);
    });

    client.lock().unwrap().batch_execute(query.as_str()).await
}