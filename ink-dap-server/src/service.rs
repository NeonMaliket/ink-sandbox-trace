use actix_web::{post, web, HttpResponse, Responder};
use serde::Deserialize;

use crate::{log::dap_log, types::DapServer};

#[derive(Deserialize, Debug)]
struct LogRequest {
    message: String,
}

#[post("/log")]
pub(crate) async fn log(
    log_req: web::Json<LogRequest>,
    server: web::Data<DapServer>,
) -> impl Responder {
    let req = log_req.into_inner();
    dap_log(server.get_ref().clone(), req.message);
    HttpResponse::Ok()
}
