mod command_handler;
mod log;
mod service;
mod state;
mod types;
mod utils;

use crate::command_handler::handle;
use crate::log::{dap_log, init_log_channel, send_log};
use crate::state::DapState;
use crate::types::DynResult;
use dap::prelude::*;
use std::io::{BufReader, BufWriter};
use std::sync::{Arc, Mutex};

fn main() -> DynResult<()> {
    let mut rx_logs = init_log_channel();

    let (req_tx, req_rx) = crossbeam_channel::unbounded::<Request>();

    std::thread::spawn(move || {
        let input = BufReader::new(std::io::stdin());
        let output = BufWriter::new(std::io::sink());
        let mut server_in = Server::new(input, output);

        loop {
            match server_in.poll_request() {
                Ok(Some(req)) => {
                    if req_tx.send(req).is_err() {
                        break;
                    }
                }
                Ok(None) => {
                    break;
                }
                Err(e) => {
                    eprintln!("[DAP] read error: {e}");
                    break;
                }
            }
        }
    });

    let input = BufReader::new(std::io::empty());
    let output = BufWriter::new(std::io::stdout());
    let server_out = Arc::new(Mutex::new(Server::new(input, output)));
    let mut state = DapState::new();

    loop {
        while let Ok(msg) = rx_logs.try_recv() {
            dap_log(Arc::clone(&server_out), msg);
        }

        match req_rx.recv_timeout(std::time::Duration::from_millis(50)) {
            Ok(req) => {
                if let Err(e) = handle(req, Arc::clone(&server_out), &mut state) {
                    eprintln!("[DAP] Error processing command: {e}");
                    send_log(format!("Error: {e}"));
                }
            }
            Err(crossbeam_channel::RecvTimeoutError::Timeout) => {}
            Err(crossbeam_channel::RecvTimeoutError::Disconnected) => {
                eprintln!("[DAP] request channel disconnected, exiting.");
                break;
            }
        }
    }

    Ok(())
}
