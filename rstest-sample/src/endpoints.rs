use actix_web::{post, HttpResponse, Responder};
use serde::{Deserialize, Serialize};

#[derive(PartialEq, Serialize, Deserialize)]
pub struct EchoResponse {
    msg: String,
}

impl EchoResponse {
    pub fn msg(&self) -> &str {
        self.msg.as_ref()
    }
}

#[post("/echo")]
pub async fn echo_msg(req_body: String) -> impl Responder {
    let response = EchoResponse { msg: req_body };
    HttpResponse::Ok().json(response)
}
