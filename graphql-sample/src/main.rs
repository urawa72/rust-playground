use actix_web::{middleware, web, App, HttpServer, Responder};
use dotenv::{dotenv, from_filename};

async fn hello_world() -> impl Responder {
    "Hello, World!"
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    if cfg!(debug_assertions) {
        from_filename(".env.local").ok();
    } else {
        dotenv().ok();
    }
    env_logger::init();

    let mut server = HttpServer::new(move || {
        App::new()
            .wrap(middleware::Logger::default())
            .route("/", web::get().to(hello_world))
    });

    server = server.bind("127.0.00.1:3000").unwrap();
    server.run().await
}
