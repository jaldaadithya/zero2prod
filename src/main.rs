use std::{net::TcpListener};

use sqlx::{MySqlConnection, Connection, MySqlPool};
use zero2prod::{startup::run, configuration::{get_configuration}};

#[tokio::main]
async fn main() -> std::io::Result<()>{
    let configuration = get_configuration().expect("Failed to read configuration");
    let connection_string = configuration.database.connection_string();
    let connection = MySqlPool::connect(&connection_string).await.expect("Failed to connec to Mysql");
    let address = format!("127.0.0.1:{}",configuration.application_port);
    let listener = TcpListener::bind(address)?;
    println!("{}",listener.local_addr().unwrap().port());
    run(listener,connection)?.await
    // tokio::spawn(server).await?
}