use actix_web::{get, App, HttpServer, Responder};
use actix_web::middleware::Logger;

#[get("/")]
async fn index() -> impl Responder {
    "Hello world"
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();
    HttpServer::new(|| App::new().wrap(Logger::default()).service(index))
        .bind("127.0.0.1:8080")?
        .run()
        .await
}
