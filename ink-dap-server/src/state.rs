use std::{collections::HashMap, net::TcpListener};

use actix_web::{App, HttpServer};
use dap::types::Source;

use crate::{service::log, types::DynResult};

const DEFAULT_PORT: u16 = 9229;
const DEFAULT_ADDRESS: &str = "127.0.0.1";

#[derive(Default, Debug)]
pub(crate) struct DapState {
    pub(crate) main_thread_id: i64,
    pub(crate) current_source: Option<Source>,
    pub(crate) stopped_line: i64,
    pub(crate) stopped_column: i64,
    pub(crate) breakpoints_by_path: HashMap<String, Vec<i64>>,
    pub(crate) vars_ref: i64,
    pub(crate) port: Option<u16>,
}

impl DapState {
    pub(crate) fn new() -> Self {
        Self {
            main_thread_id: 1,
            stopped_line: 1,
            stopped_column: 1,
            vars_ref: 2000,
            ..Default::default()
        }
    }

    pub(crate) fn pick_stop_location(&mut self) {
        if let Some(src) = &self.current_source {
            if let Some(path) = &src.path {
                if let Some(lines) = self.breakpoints_by_path.get(path) {
                    if let Some(first) = lines.first() {
                        self.stopped_line = *first;
                        self.stopped_column = 1;
                        return;
                    }
                }
            }
        }
        self.stopped_line = 1;
        self.stopped_column = 1;
    }

    pub(crate) fn run_server(&self) -> DynResult<()> {
        let port = self.port.unwrap_or(DEFAULT_PORT);

        let listener = TcpListener::bind((DEFAULT_ADDRESS, port))?;

        std::thread::spawn(move || {
            let sys = actix_web::rt::System::new();
            let _ = sys.block_on(async move {
                HttpServer::new(move || App::new().service(log))
                    .listen(listener)?
                    .run()
                    .await
            });
        });

        Ok(())
    }
}
