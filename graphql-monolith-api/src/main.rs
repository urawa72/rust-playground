use std::sync::Mutex;

use actix_web::{
    guard,
    web::{self, Data},
    App, HttpResponse, HttpServer,
};
use async_graphql::{
    http::{playground_source, GraphQLPlaygroundConfig},
    EmptySubscription, Object, Schema,
};
use async_graphql_actix_web::{GraphQLRequest, GraphQLResponse};
use once_cell::sync::Lazy;

static PHOTOS: Lazy<Mutex<Vec<Photo>>> = Lazy::new(|| Mutex::new(vec![]));

struct Photo {
    name: String,
    description: String,
}

struct Mutation;

#[Object]
impl Mutation {
    async fn post_photo(&self, name: String, description: String) -> bool {
        let photo = Photo { name, description };
        PHOTOS.lock().unwrap().push(photo);
        true
    }
}

struct Query;

#[Object]
impl Query {
    async fn total_photos(&self) -> usize {
        PHOTOS.lock().unwrap().len()
    }
}

type ApiSchema = Schema<Query, Mutation, EmptySubscription>;

async fn index(schema: web::Data<ApiSchema>, req: GraphQLRequest) -> GraphQLResponse {
    schema.execute(req.into_inner()).await.into()
}

async fn index_playground() -> actix_web::Result<HttpResponse> {
    let source = playground_source(GraphQLPlaygroundConfig::new("/").subscription_endpoint("/"));
    Ok(HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(source))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let schema = Schema::build(Query, Mutation, EmptySubscription).finish();
    println!("Playground: http://localhost:8000");
    HttpServer::new(move || {
        App::new()
            .app_data(Data::new(schema.clone()))
            .service(web::resource("/").guard(guard::Post()).to(index))
            .service(web::resource("/").guard(guard::Get()).to(index_playground))
    })
    .bind("127.0.0.1:8000")?
    .run()
    .await
}
