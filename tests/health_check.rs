use std::{net::TcpListener, io::{stdout, sink}};

use once_cell::sync::Lazy;
use secrecy::ExposeSecret;
use sqlx::{MySqlPool, MySqlConnection, Connection, Executor};
use uuid::Uuid;
use zero2prod::{configuration::{get_configuration}, telemetry::{get_subscriber, init_subscriber}};

static TRACING: Lazy<()> = Lazy::new(|| {
     if std::env::var("TEST_LOG").is_ok() {
        let subscriber = get_subscriber("test".into(), "debug".into(),stdout);
        init_subscriber(subscriber);
            } else {
                let subscriber = get_subscriber("test".into(), "debug".into(),sink);
                init_subscriber(subscriber);
            };
});

#[tokio::test]
async fn health_check_works() {
// Arrange
let app = spawn_app().await; // We need to bring in `reqwest`
// to perform HTTP requests against our application. 
let client = reqwest::Client::new();
// Act
    let response = client
        .get(&format!("{}/health_check", &app.address))
        .send()
        .await
        .expect("Failed to execute request.");
// Assert
assert!(response.status().is_success());
assert_eq!(Some(0), response.content_length()); 
}

#[tokio::test]
async fn subscribe_returns_a_200_for_valid_form_data() {
    // let configuration = get_configuration().expect("Failed to read configurations");
    // let connection_string = configuration.database.connection_string();
    // let mut connection = MySqlPool::connect(&connection_string).await.expect("Failed to connec to Mysql");
    let app = spawn_app().await;
    // try to connect to mysql
    let client = reqwest::Client::new();
    let body = "name=&email=jaldaadithya%40gmail.com";

    let response = client.post(&format!("{}/subscriptions",&app.address))
    .header("Content-type", "application/x-www-form-urlencoded")
    .body(body)
    .send()
    .await.expect("Failed to execute request");
    let status = response.status().as_u16();
    assert_eq!(200,status);
    let saved = sqlx::query!("select email,name from subscriptions")
    .fetch_one(&app.db_pool)
    .await.expect("Failed to fetch saved subscriptions");
    assert_eq!(saved.email, "jaldaadithya@gmail.com");
    assert_eq!(saved.name, "adithya");
}

#[tokio::test]
async fn subscribe_returns_a_400_when_data_is_missing() {
// Arrange
let app = spawn_app().await;
let client = reqwest::Client::new(); let test_cases = vec![
        ("name=", "missing the email"),
        ("email=ursula_le_guin%40gmail.com", "missing the name"),
        ("", "missing both name and email")
];
    for (invalid_body, error_message) in test_cases {
        // Act
let response = client
.post(&format!("{}/subscriptions", &app.address)) .header("Content-Type", "application/x-www-form-urlencoded")
.body(invalid_body)
    .send()
    .await
    .expect("Failed to execute request.");
    assert_eq!( 400,
        response.status().as_u16(),
        // Additional customised error message on test failure
        "The API did not fail with 400 Bad Request when the payload was {}.",
        error_message);
}
}
    
pub struct TestApp {
    pub address: String,
    pub db_pool: MySqlPool,
}

// Launch our application in the background ~somehow~
async fn spawn_app() -> TestApp { 

    Lazy::force(&TRACING);
    
    let listener = TcpListener::bind("127.0.0.1:0")
    .expect("Failed to bind random port");
    let port = listener.local_addr().unwrap().port();
    let mut configuration = get_configuration().expect("Failed to read configurations");
    configuration.database.database_name = Uuid::new_v4().to_simple().to_string();
    // let connection_string = configuration.database.connection_string();
    let connection = configure_database(&configuration.database).await;
    // let connection = MySqlPool::connect(&connection_string).await.expect("Failed to connec to Mysql");
    let server = zero2prod::startup::run(listener,connection.clone()).expect("Failed to bind address");
    let address = format!("http://127.0.0.1:{}", port);
    let _ = tokio::spawn(server);
    TestApp {
        address,
        db_pool: connection,
    }
}

async fn configure_database(database: &zero2prod::configuration::DatabaseSettings) -> MySqlPool {
    let mut connection = MySqlConnection::connect(&database.connection_string_without_db().expose_secret())
    .await
    .expect("Failed to connect to Mysql");

    connection.execute(format!(r#"CREATE database `{}`;"#,database.database_name).as_str())
    .await
    .expect("Failed to create database");

    let pool = MySqlPool::connect(&database.connection_string().expose_secret())
    .await
    .expect("Fialed to connect to Mysql");

    sqlx::migrate!("./migrations").run(&pool).await.expect("Failed to run migrations");

    pool
}