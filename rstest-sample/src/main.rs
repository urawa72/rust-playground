pub mod endpoints;
use actix_web::{App, HttpServer};
use endpoints::echo_msg;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(echo_msg))
        .bind("127.0.0.1:8080")?
        .run()
        .await
}

#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::{http, test, App};
    use rstest::rstest;

    #[rstest(msg)]
    #[case("Hello")]
    #[case("World")]
    #[actix_rt::test]
    async fn test_eche(msg: &str) {
        let mut app = test::init_service(App::new().service(echo_msg)).await;
        let res = test::TestRequest::post()
            .uri("/echo")
            .set_payload(msg.to_string())
            .send_request(&mut app)
            .await;

        assert_eq!(res.status(), http::StatusCode::OK);
        let body: endpoints::EchoResponse = test::read_body_json(res).await;
        assert_eq!(body.msg(), msg);
    }
}
