use std::sync::OnceLock;

use dap::{
    events::{Event, OutputEventBody},
    server::Server,
    types::OutputEventCategory,
};
use tokio::sync::mpsc::{unbounded_channel, UnboundedReceiver, UnboundedSender};

static LOG_TX: OnceLock<UnboundedSender<String>> = OnceLock::new();

pub(crate) fn init_log_channel() -> UnboundedReceiver<String> {
    let (tx, rx) = unbounded_channel();
    let _ = LOG_TX.set(tx);
    rx
}

pub(crate) fn send_log(msg: impl Into<String>) {
    if let Some(tx) = LOG_TX.get() {
        let _ = tx.send(msg.into());
    }
}

pub(crate) fn dap_log<S: std::io::Read, W: std::io::Write>(
    server: &mut Server<S, W>,
    msg: impl AsRef<str>,
) {
    let _ = server.send_event(Event::Output(OutputEventBody {
        category: Some(OutputEventCategory::Console),
        output: format!("{}\n", msg.as_ref()),
        ..Default::default()
    }));
}
