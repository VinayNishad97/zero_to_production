use actix_web::{App, HttpServer, dev::Server, web::{self}};
use std::net::TcpListener;
use sqlx::{ PgPool};
use tracing_actix_web::TracingLogger;
use crate::routes::{health_check::health_check
    ,subscription::subscription};
pub fn run(listner:TcpListener,
connection_pool:PgPool
) -> Result<Server, std::io::Error>{
    let connection = web::Data::new(connection_pool);
 let server = HttpServer::new(move|| {

App::new()
.wrap(TracingLogger::default())
.route("/healthcheck", web::get().to(health_check))
.route("/", web::get().to(greeting))
.route("/subscription", web::post().to(subscription))
.app_data(connection.clone())
})
.listen(listner)?
.run();

Ok(server)
}

pub async fn greeting()->String{
    format!("hello there")
}
