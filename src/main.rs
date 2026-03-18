use secrecy::ExposeSecret;
use std::net::TcpListener;
pub mod routes;
pub mod startup; 
pub mod configration;
use sqlx::{ PgPool};
use startup::run;
pub use configration::get_configuration;
use zero_to_production::telemetry::{get_subscriber,init_subscriber};
#[tokio::main]
async fn main() -> std::io::Result<()> {
let subscriber  = get_subscriber("zero_to_production".into(), "info".into(),std::io::stdout);
init_subscriber(subscriber);
let configuration = get_configuration().expect("failed to connect");
let connection_pool = PgPool::connect(&configuration.database.connecting_string().expose_secret())
.await
.expect("there is an issue while connecting to database");
let address = format!("127.0.0.1:{}",configuration.application_port);
let listner = TcpListener::bind(address)?;

    run(listner,connection_pool)?.await

}
