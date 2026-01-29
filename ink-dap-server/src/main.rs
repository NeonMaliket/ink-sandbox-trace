mod command_handler;
mod log;
mod service;
mod state;
mod types;
mod utils;
use crate::command_handler::handle;
use crate::log::dap_log;
use crate::state::DapState;
use crate::types::DynResult;
use dap::prelude::*;
use std::{
    io::{BufReader, BufWriter},
    sync::{Arc, Mutex},
};

fn main() -> DynResult<()> {
    let output = BufWriter::new(std::io::stdout());
    let input = BufReader::new(std::io::stdin());
    let mut state = DapState::new();
    let server = Arc::new(Mutex::new(Server::new(input, output)));

    loop {
        let req = {
            let server = Arc::clone(&server);
            let mut server = server.lock().map_err(|_| {
              "Failed to acquire lock on DAP server. This is likely a bug in the DAP server implementation."
           })?;
            match server.poll_request()? {
                Some(req) => req,
                None => {
                    eprintln!("No request received, exiting.");
                    break;
                }
            }
        };

        let result: DynResult<()> = handle(req, Arc::clone(&server), &mut state);

        if let Err(e) = result {
            eprintln!("[DAP] Error processing command: {}", e);
            dap_log(Arc::clone(&server), format!("Error: {}", e));
        }
    }

    Ok(())
}
