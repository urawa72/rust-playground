use std::{env, io};

use actix_web::{middleware, App, HttpServer};
use todos::{db::get_pool, endpoints::graphql_endpoints};

#[actix_rt::main]
async fn main() -> io::Result<()> {
    logging_setup();

    let pool = get_pool();

    let mut server = HttpServer::new(move || {
        App::new()
            .data(pool.clone())
            .wrap(middleware::Logger::default())
            .configure(graphql_endpoints)
    });
    server = server.bind("127.0.00.1:3000")?;
    server.run().await
}

fn logging_setup() {
    env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();
}
