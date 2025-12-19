use dap::{
    events::{Event, OutputEventBody},
    server::Server,
    types::OutputEventCategory,
};

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
