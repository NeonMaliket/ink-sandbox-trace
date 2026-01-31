use std::sync::Arc;

use actix_web::{post, web, HttpResponse, Responder};
use dap::{events::Event, types::StoppedEventReason};
use serde::Deserialize;

use crate::{
    command_handler::server_send_event,
    log::send_log,
    types::{DapServerOut, SharedDapState},
};

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

#[post("/pause")]
pub(crate) async fn pause(
    server: web::Data<DapServerOut>,
    state: web::Data<SharedDapState>,
) -> impl Responder {
    send_log("Pause command received");
    let _ = state.lock().map(|state| {
        let _ = server_send_event(
            Arc::clone(&server),
            Event::Stopped(dap::events::StoppedEventBody {
                reason: StoppedEventReason::Pause,
                description: Some("Paused".to_string()),
                thread_id: Some(state.main_thread_id),
                preserve_focus_hint: Some(false),
                text: None,
                all_threads_stopped: Some(true),
                hit_breakpoint_ids: None,
            }),
        );
    });
    HttpResponse::Ok()
}
