use actix_web::{post, web, HttpResponse, Responder};
use serde::Deserialize;

use crate::log::send_log;

#[derive(Deserialize, Debug)]
struct LogRequest {
    message: String,
}

#[post("/log")]
pub(crate) async fn log(log_req: web::Json<LogRequest>) -> impl Responder {
    let req = log_req.into_inner();
    send_log(req.message);
    HttpResponse::Ok()
}
