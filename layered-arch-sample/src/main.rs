use actix_web::{App, HttpServer};
use layered_arch_sample::presentations::{
    controller::documents::documents_controller, request_context::RequestContext,
};

#[actix_web::main]
async fn run() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .data(RequestContext::new())
            .service(documents_controller::get_documents)
            .service(documents_controller::get_document)
            .service(documents_controller::post_document)
            .service(documents_controller::delete_document)
            .service(documents_controller::update_document)
    })
    .bind("127.0.0.1:3000")?
    .run()
    .await
}

fn main() -> std::io::Result<()> {
    run()
}
