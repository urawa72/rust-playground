use std::{env, io};

use actix_web::{middleware, web, App, HttpServer, Responder};
use todos::{db::get_pool, endpoints::graphql_endpoints};

async fn hello_world() -> impl Responder {
    "Hello, World!"
}

#[actix_rt::main]
async fn main() -> io::Result<()> {
    logging_setup();

    let pool = get_pool();

    let mut server = HttpServer::new(move || {
        App::new()
            .data(pool.clone())
            .wrap(middleware::Logger::default())
            .route("/", web::get().to(hello_world))
            .configure(graphql_endpoints)
    });
    server = server.bind("127.0.00.1:3000")?;
    server.run().await
}

fn logging_setup() {
    env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();
}
