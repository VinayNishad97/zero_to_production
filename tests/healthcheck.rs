use std::net::TcpListener;
//dynamic8460@gmail.com
//mp40m1887
use sqlx::{Connection, PgConnection,Executor};
use uuid::Uuid;
use zero_to_production::{configration::{self, DatabaseSettings}, startup::run};
use sqlx::PgPool;
use once_cell::sync::Lazy;
use zero_to_production::telemetry::{get_subscriber,init_subscriber};
use secrecy::ExposeSecret;
#[tokio::test]
async fn health_check_works() {
    let address = spawn_app().await;
    let cleint = reqwest::Client::new();
    let response = cleint
        .get(&format!("{}/healthcheck", &address.address))
        .send()
        .await
        .expect("failed");
    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}
static TRACING: Lazy<()> = Lazy::new(|| {
let default_filter_level = "info".to_string();
let subscriber_name = "test".to_string();
// We cannot assign the output of `get_subscriber` to a variable based on the value of `TEST_LOG`
// because the sink is part of the type returned by `get_subscriber`, therefore they are not the
// same type. We could work around it, but this is the most straight-forward way of moving forward.
if std::env::var("TEST_LOG").is_ok() {
let subscriber = get_subscriber(subscriber_name, default_filter_level, std::io::stdout);
init_subscriber(subscriber);
} else {
let subscriber = get_subscriber(subscriber_name, default_filter_level, std::io::sink);
init_subscriber(subscriber);
};
});
pub struct Testapp{
    pub address:String,
    pub pool:PgPool
}

async fn spawn_app() -> Testapp {  
    Lazy::force(&TRACING);
    let lisner = TcpListener::bind("127.0.0.1:0").expect("failed");
    let port = lisner.local_addr().unwrap().port();
    let address =  format!("http://127.0.0.1:{}", port);
let mut configration =configration::get_configuration().expect("failed to get configuration");
configration.database.database_name = Uuid::new_v4().to_string();
let connection_pool = configure_database(&configration.database).await;
 let server = run(lisner, connection_pool.clone()).expect("failed");
    let _ = tokio::spawn(server);

  Testapp{
    address,
    pool:connection_pool
  }
}

#[tokio::test]

async fn return_a_200_if_the_user_input_is_correct() {
    let address = spawn_app().await;
    let configuration = configration::get_configuration().expect("failed to get configurations");
    let connecting_string = configuration.database.connecting_string();
    let mut connection = PgConnection::connect(&connecting_string.expose_secret())
    .await
    .expect("failed to connect to the postgress");
    let body = "name=le%20guin&email=ursula_le_guin%40gmail.com";
    let client = reqwest::Client::new();



    let response = client
        .post(&format!("{}/subscription", &address.address))
        .header("Content-Type", "application/x-www-form-urlencoded")
        .body(body)
        .send()
        .await
        .expect("failed");

    assert_eq!(200, response.status().as_u16());
    let saved = sqlx::query!("SELECT email, name FROM subscriptions",)
.fetch_one(&mut connection)
.await
.expect("Failed to fetch saved subscription.");
assert_eq!(saved.email, "ursula_le_guin@gmail.com");
assert_eq!(saved.name, "le guin");
}

#[tokio::test]
async fn subscribe_returns_a_400_when_data_is_missing() {
    // Arrange
    let app_address = spawn_app().await;
    let client = reqwest::Client::new();
    let test_cases = vec![
        ("name=le%20guin", "missing the email"),
        ("email=ursula_le_guin%40gmail.com", "missing the name"),
        ("", "missing both name and email"),
    ];
    for (invalid_body, error_message) in test_cases {
        // Act
        
        let response = client
            .post(&format!("{}/subscription", &app_address.address))
            .header("Content-Type", "application/x-www-form-urlencoded")
            .body(invalid_body)
            .send()
            .await
            .expect("Failed to execute request.");
        // Assert
        assert_eq!(
            400,
            response.status().as_u16(),
            // Additional customised error message on test failure
            "The API did not fail with 400 Bad Request when the payload was {}.",
            error_message
        );
    }
}

pub async fn configure_database(config: &DatabaseSettings) -> PgPool {
// Create database
let mut connection = PgConnection::connect(&config.connection_string_without_db())
.await
.expect("Failed to connect to Postgres");
connection
.execute(format!(r#"CREATE DATABASE "{}";"#, config.database_name).as_str())
.await
.expect("Failed to create database.");
// Migrate database
let connection_pool = PgPool::connect(&config.connecting_string().expose_secret())
.await
.expect("Failed to connect to Postgres.");
sqlx::migrate!("./migrations")
.run(&connection_pool)
.await
.expect("Failed to migrate the database");
connection_pool
}