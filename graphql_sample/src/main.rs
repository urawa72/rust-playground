use std::sync::Arc;

use crate::graphql::schema::{create_shema, Photo};
use actix_web::{middleware, web, App, Error, HttpResponse, HttpServer, Responder};
use dotenv::{dotenv, from_filename};
use graphql::schema::{Context, Schema};
use juniper::{graphiql::graphiql_source, http::GraphQLRequest};

pub mod graphql;

impl Photo {
    fn new(id: String, name: String, description: String) -> Photo {
        Photo {
            id,
            name,
            description,
        }
    }
}

/// handler for actix to access grahql
pub async fn graphql(
    st: web::Data<Arc<Schema>>,
    data: web::Json<GraphQLRequest>,
) -> Result<HttpResponse, Error> {
    let user = web::block(move || {
        let res = data.execute(
            &st,
            &Context {
                photos: vec![Photo::new(
                    "1".to_string(),
                    "test1".to_string(),
                    "test1 photo".to_string(),
                )],
            },
        );
        Ok::<_, serde_json::error::Error>(serde_json::to_string(&res)?)
    })
    .await?;
    Ok(HttpResponse::Ok()
        .content_type("application/json")
        .body(user))
}

/// handler for actix to access graphi
pub async fn graphiql() -> HttpResponse {
    let html = graphiql_source("http://127.0.0.1:3000/graphql");
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(html)
}

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

    let schema = Arc::new(create_shema());

    let mut server = HttpServer::new(move || {
        App::new()
            .data(schema.clone())
            .wrap(middleware::Logger::default())
            .route("/", web::get().to(hello_world))
            .route("/graphiql", web::get().to(graphiql))
            .route("/graphql", web::post().to(graphql))
    });

    server = server.bind("127.0.00.1:3000").unwrap();
    server.run().await
}
