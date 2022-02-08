use std::{net::TcpListener, fmt::format, io::Bytes};

use reqwest::Client;


#[tokio::test]
async fn health_check_works() {
// Arrange
let address = spawn_app(); // We need to bring in `reqwest`
// to perform HTTP requests against our application. 
let client = reqwest::Client::new();
// Act
    let response = client
        .get(&format!("{}/health_check", &address))
        .send()
        .await
        .expect("Failed to execute request.");
// Assert
assert!(response.status().is_success());
assert_eq!(Some(0), response.content_length()); }

#[tokio::test]
async fn subscribe_returns_a_200_for_valid_form_data() {
    let app_address = spawn_app();
    let client = reqwest::Client::new();
    let body = "name=jalda%20Adithya&email=jaldaadithya%40gmail.com";

    let response = client.post(&format!("{}/subscriptions",&app_address))
    .header("Content-type", "application/x-www-form-urlencoded")
    .body(body)
    .send()
    .await.expect("Failed to execute request");
    let status = response.status().as_u16();
    assert_eq!(200,status);
}

#[tokio::test]
async fn subscribe_returns_a_400_when_data_is_missing() {
// Arrange
let app_address = spawn_app();
let client = reqwest::Client::new(); let test_cases = vec![
        ("name=le%20guin", "missing the email"),
        ("email=ursula_le_guin%40gmail.com", "missing the name"),
        ("", "missing both name and email")
];
    for (invalid_body, error_message) in test_cases {
        // Act
let response = client
.post(&format!("{}/subscriptions", &app_address)) .header("Content-Type", "application/x-www-form-urlencoded")
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
    
// Launch our application in the background ~somehow~
fn spawn_app() -> String { 
    let listener = TcpListener::bind("127.0.0.1:0")
    .expect("Failed to bind random port");
    let port = listener.local_addr().unwrap().port();
    let server = zero2prod::run(listener).expect("Failed to bind address");

    let _ = tokio::spawn(server);
    format!("http://127.0.0.1:{}", port)
}