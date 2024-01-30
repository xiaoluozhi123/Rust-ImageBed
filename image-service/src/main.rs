mod routers;

use actix_web::{App, HttpServer};
use std::io;

#[actix_web::main]
async fn main() -> io::Result<()> {
    HttpServer::new(|| App::new().service(routers::config::home_config))
        .bind("127.0.0.1:8080")?
        .run()
        .await
}
