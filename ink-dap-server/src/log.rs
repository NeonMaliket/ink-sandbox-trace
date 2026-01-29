use dap::{
    events::{Event, OutputEventBody},
    types::OutputEventCategory,
};

use crate::types::DapServer;

pub(crate) fn dap_log(server: DapServer, msg: impl AsRef<str>) {
    let _ = server.lock().map(|mut server| {
        let _ = server.send_event(Event::Output(OutputEventBody {
            category: Some(OutputEventCategory::Console),
            output: format!("{}\n", msg.as_ref()),
            ..Default::default()
        }));
    });
}
