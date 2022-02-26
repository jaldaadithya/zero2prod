use std::{net::TcpListener, io::stdout, time::Duration};

use secrecy::ExposeSecret;
use sqlx::{MySqlPool, mysql::MySqlPoolOptions};
use zero2prod::{startup::run, configuration::{get_configuration}, telemetry::{get_subscriber, init_subscriber}};
// use zero2prod::telemetry::get_subscriber;

#[tokio::main]
async fn main() -> std::io::Result<()>{

    let subscriber = get_subscriber("zero2prod".into(), "info".into(),stdout);
    init_subscriber(subscriber);

    let configuration = get_configuration().expect("Failed to read configuration");
    let connection_string = configuration.database.connection_string();
    let connection = MySqlPoolOptions::new().connect_timeout(Duration::from_secs(2)).connect_lazy(&connection_string.expose_secret()).expect("Failed to connec to Mysql");
    let address = format!("{}:{}",configuration.application.host,configuration.application.port);
    let listener = TcpListener::bind(address)?;
    println!("{}",listener.local_addr().unwrap().port());
    run(listener,connection)?.await
    // tokio::spawn(server).await?
}